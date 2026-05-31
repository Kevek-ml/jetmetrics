use arrow::array::{ArrayRef, Float64Array};
use arrow::compute::{cast, max, min};
use arrow::datatypes::DataType;

use crate::error::{JetmetricsError, Result};

/// Equal-width histogram over a float array.
///
/// Shared by PSI, Hellinger, TVD, and JS divergence.
/// Bin edges are derived from the union of current + reference min/max
/// so that both arrays use the same bins.
pub struct Histogram {
    pub bin_edges: Vec<f64>,
    pub counts: Vec<u64>,
    /// True when all non-null values in the array were identical (min == max).
    /// Distance metrics must return 0.0 for a degenerate pair without inspecting counts.
    pub is_degenerate: bool,
}

impl Histogram {
    /// Build a histogram with `n_bins` equal-width bins over `[min, max]`.
    ///
    /// `min` and `max` must be pre-computed via `shared_edges` so that current
    /// and reference arrays use identical bin boundaries.
    ///
    /// When `min == max` (constant-valued array), returns a degenerate single-bin
    /// histogram with `is_degenerate = true`. Distance metrics should short-circuit
    /// to 0.0 when both histograms are degenerate at the same value.
    ///
    /// Values outside `[min, max]` are clamped into the nearest boundary bin.
    pub fn build(array: &ArrayRef, n_bins: usize, min: f64, max: f64) -> Result<Self> {
        if n_bins == 0 {
            return Err(JetmetricsError::InvalidInput(
                "n_bins must be > 0".to_string(),
            ));
        }
        if min > max {
            return Err(JetmetricsError::InvalidInput(format!(
                "min ({min}) must be <= max ({max})"
            )));
        }

        // Degenerate case: constant-valued array.
        if min == max {
            let f64_array = cast(array, &DataType::Float64)?;
            let values = f64_array
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| {
                    JetmetricsError::InvalidInput("expected numeric array".to_string())
                })?;
            let count = values.iter().flatten().count() as u64;
            return Ok(Self {
                bin_edges: vec![min, min],
                counts: vec![count],
                is_degenerate: true,
            });
        }

        let f64_array = cast(array, &DataType::Float64)?;
        let values = f64_array
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| JetmetricsError::InvalidInput("expected numeric array".to_string()))?;

        let mut counts = vec![0u64; n_bins];

        for val in values.iter().flatten() {
            // Fractional position in [0, n_bins]. Using division by range avoids
            // progressive drift from repeated bin_width multiplication.
            let frac = (val - min) / (max - min) * n_bins as f64;
            // Clamp: values exactly at max map to n_bins (out of bounds), pulled back.
            // Values below min clamp to 0.
            let idx = (frac as usize).min(n_bins - 1);
            counts[idx] += 1;
        }

        // Edges computed as fractions of range to match numpy linspace precision.
        let bin_edges = (0..=n_bins)
            .map(|i| min + (i as f64 / n_bins as f64) * (max - min))
            .collect();

        Ok(Self {
            bin_edges,
            counts,
            is_degenerate: false,
        })
    }

    /// Normalize counts to a probability distribution (sums to 1).
    ///
    /// Returns all-zeros when the array contained no non-null values.
    /// Callers computing ratios (e.g. PSI) must apply an epsilon floor
    /// to avoid division by zero — that is a metric-level concern, not handled here.
    pub fn normalize(&self) -> Vec<f64> {
        let total: u64 = self.counts.iter().sum();
        if total == 0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts
            .iter()
            .map(|&c| c as f64 / total as f64)
            .collect()
    }
}

/// Compute the union [min, max] range across two arrays.
///
/// Used to derive shared bin edges so both arrays are binned identically.
/// Returns `(min, max)` where min == max when all values in both arrays are identical.
pub fn shared_edges(a: &ArrayRef, b: &ArrayRef) -> Result<(f64, f64)> {
    let a_f64 = cast(a, &DataType::Float64)?;
    let b_f64 = cast(b, &DataType::Float64)?;

    let a_vals = a_f64
        .as_any()
        .downcast_ref::<Float64Array>()
        .ok_or_else(|| JetmetricsError::InvalidInput("expected numeric array".to_string()))?;
    let b_vals = b_f64
        .as_any()
        .downcast_ref::<Float64Array>()
        .ok_or_else(|| JetmetricsError::InvalidInput("expected numeric array".to_string()))?;

    let min_val = [min(a_vals), min(b_vals)]
        .into_iter()
        .flatten()
        .fold(f64::INFINITY, f64::min);

    let max_val = [max(a_vals), max(b_vals)]
        .into_iter()
        .flatten()
        .fold(f64::NEG_INFINITY, f64::max);

    if min_val.is_infinite() || max_val.is_infinite() {
        return Err(JetmetricsError::InvalidInput(
            "arrays are empty or contain only nulls".to_string(),
        ));
    }

    Ok((min_val, max_val))
}

#[cfg(test)]
mod tests {
    use arrow::array::Float64Array;
    use std::sync::Arc;

    use super::*;

    fn f64_array(vals: &[f64]) -> ArrayRef {
        Arc::new(Float64Array::from(vals.to_vec()))
    }

    #[test]
    fn test_shared_edges_union_range() {
        let a = f64_array(&[1.0, 2.0, 3.0]);
        let b = f64_array(&[0.0, 5.0]);
        let (min, max) = shared_edges(&a, &b).unwrap();
        assert_eq!(min, 0.0);
        assert_eq!(max, 5.0);
    }

    #[test]
    fn test_build_counts_sum_to_n() {
        let a = f64_array(&[0.0, 1.0, 2.0, 3.0, 4.0]);
        let hist = Histogram::build(&a, 5, 0.0, 5.0).unwrap();
        assert_eq!(hist.counts.iter().sum::<u64>(), 5);
    }

    #[test]
    fn test_normalize_sums_to_one() {
        let a = f64_array(&[0.5, 1.5, 2.5, 3.5, 4.5]);
        let hist = Histogram::build(&a, 5, 0.0, 5.0).unwrap();
        let probs = hist.normalize();
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_max_value_clamped_to_last_bin() {
        let a = f64_array(&[5.0]);
        let hist = Histogram::build(&a, 5, 0.0, 5.0).unwrap();
        assert_eq!(hist.counts[4], 1);
        assert_eq!(hist.counts.iter().sum::<u64>(), 1);
    }

    #[test]
    fn test_min_value_clamped_to_first_bin() {
        let a = f64_array(&[-1.0]); // below the declared min
        let hist = Histogram::build(&a, 5, 0.0, 5.0).unwrap();
        assert_eq!(hist.counts[0], 1);
        assert_eq!(hist.counts.iter().sum::<u64>(), 1);
    }

    #[test]
    fn test_bin_edges_count_and_bounds() {
        let a = f64_array(&[1.0]);
        let hist = Histogram::build(&a, 10, 0.0, 10.0).unwrap();
        assert_eq!(hist.bin_edges.len(), 11);
        assert_eq!(hist.bin_edges[0], 0.0);
        assert_eq!(hist.bin_edges[10], 10.0);
    }

    #[test]
    fn test_degenerate_constant_array() {
        let a = f64_array(&[3.0, 3.0, 3.0]);
        let b = f64_array(&[3.0]);
        let hist = Histogram::build(&a, 10, 3.0, 3.0).unwrap();
        assert!(hist.is_degenerate);
        assert_eq!(hist.counts[0], 3);
        // shared_edges returns (3.0, 3.0) for constant arrays
        let (min, max) = shared_edges(&a, &b).unwrap();
        assert_eq!(min, max);
    }

    #[test]
    fn test_error_on_empty_arrays() {
        let a = f64_array(&[]);
        let b = f64_array(&[]);
        assert!(shared_edges(&a, &b).is_err());
    }

    #[test]
    fn test_error_on_zero_bins() {
        let a = f64_array(&[1.0, 2.0]);
        assert!(Histogram::build(&a, 0, 0.0, 3.0).is_err());
    }

    #[test]
    fn test_normalize_all_null_returns_zeros() {
        let a = f64_array(&[1.0]);
        let hist = Histogram::build(&a, 5, 0.0, 5.0).unwrap();
        // Manually zero out counts to simulate all-null scenario
        let empty = Histogram {
            bin_edges: hist.bin_edges,
            counts: vec![0u64; 5],
            is_degenerate: false,
        };
        let probs = empty.normalize();
        assert!(probs.iter().all(|&p| p == 0.0));
    }
}
