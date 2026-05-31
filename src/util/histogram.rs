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
}

impl Histogram {
    /// Build a histogram with `n_bins` equal-width bins over `[min, max]`.
    ///
    /// `min` and `max` must be pre-computed via `shared_edges` so that current
    /// and reference arrays use identical bin boundaries.
    pub fn build(array: &ArrayRef, n_bins: usize, min: f64, max: f64) -> Result<Self> {
        if n_bins == 0 {
            return Err(JetmetricsError::InvalidInput(
                "n_bins must be > 0".to_string(),
            ));
        }
        if min >= max {
            return Err(JetmetricsError::InvalidInput(format!(
                "min ({min}) must be < max ({max})"
            )));
        }

        let f64_array = cast(array, &DataType::Float64)?;
        let values = f64_array
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| JetmetricsError::InvalidInput("expected numeric array".to_string()))?;

        let bin_width = (max - min) / n_bins as f64;
        let mut counts = vec![0u64; n_bins];

        for val in values.iter().flatten() {
            // Clamp max value into last bin (closed right edge).
            let idx = ((val - min) / bin_width) as usize;
            let idx = idx.min(n_bins - 1);
            counts[idx] += 1;
        }

        let bin_edges = (0..=n_bins)
            .map(|i| min + i as f64 * bin_width)
            .collect();

        Ok(Self { bin_edges, counts })
    }

    /// Normalize counts to a probability distribution (sums to 1).
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
        let a = f64_array(&[5.0]); // exactly at max
        let hist = Histogram::build(&a, 5, 0.0, 5.0).unwrap();
        assert_eq!(hist.counts[4], 1);
        assert_eq!(hist.counts.iter().sum::<u64>(), 1);
    }

    #[test]
    fn test_bin_edges_count() {
        let a = f64_array(&[1.0]);
        let hist = Histogram::build(&a, 10, 0.0, 10.0).unwrap();
        assert_eq!(hist.bin_edges.len(), 11); // n_bins + 1 edges
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
}
