use arrow::array::ArrayRef;

use crate::error::Result;
use crate::util::histogram::{shared_edges, Histogram};

/// Minimum probability applied to each bin before computing ln(p/q).
/// Prevents division-by-zero and ln(0) on empty bins.
/// Consistent with standard PSI implementations (1e-4 floor).
const EPSILON: f64 = 1e-4;

/// Population Stability Index between `current` and `reference` distributions.
///
/// PSI = Σ (p_i - q_i) * ln(p_i / q_i)
///
/// Both arrays are binned using shared edges derived from their union range,
/// so the same bin boundaries apply to both distributions.
///
/// Returns 0.0 when all values in both arrays are identical (degenerate case).
///
/// Empty bins receive an epsilon floor of 1e-4 before log computation —
/// this is the standard PSI convention and matches reference implementations.
pub fn psi(current: &ArrayRef, reference: &ArrayRef, n_bins: usize) -> Result<f64> {
    let (min, max) = shared_edges(current, reference)?;

    // Constant-valued feature: no drift by definition.
    if min == max {
        return Ok(0.0);
    }

    let current_hist = Histogram::build(current, n_bins, min, max)?;
    let reference_hist = Histogram::build(reference, n_bins, min, max)?;

    let current_probs = current_hist.normalize();
    let reference_probs = reference_hist.normalize();

    let psi_value = current_probs
        .iter()
        .zip(reference_probs.iter())
        .map(|(&p, &q)| {
            let p = f64::max(p, EPSILON);
            let q = f64::max(q, EPSILON);
            (p - q) * (p / q).ln()
        })
        .sum();

    Ok(psi_value)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::Float64Array;

    use super::*;

    fn f64_array(vals: &[f64]) -> ArrayRef {
        Arc::new(Float64Array::from(vals.to_vec()))
    }

    #[test]
    fn test_psi_identical_distributions() {
        let a = f64_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = psi(&a, &a, 5).unwrap();
        // Identical distributions → p_i == q_i in every bin → PSI = 0
        assert!(result < 1e-10, "PSI of identical distributions should be ~0, got {result}");
    }

    #[test]
    fn test_psi_constant_feature_returns_zero() {
        let a = f64_array(&[3.0, 3.0, 3.0, 3.0]);
        let b = f64_array(&[3.0, 3.0]);
        let result = psi(&a, &b, 10).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_psi_nonnegative() {
        let current = f64_array(&[1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5]);
        let reference = f64_array(&[3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5]);
        let result = psi(&current, &reference, 5).unwrap();
        assert!(result >= 0.0, "PSI must be non-negative, got {result}");
    }

    #[test]
    fn test_psi_symmetric_shift_increases_with_divergence() {
        // A larger shift between distributions should produce a larger PSI.
        let reference = f64_array(&(1..=100).map(|x| x as f64).collect::<Vec<_>>());
        let small_shift = f64_array(&(11..=110).map(|x| x as f64).collect::<Vec<_>>());
        let large_shift = f64_array(&(51..=150).map(|x| x as f64).collect::<Vec<_>>());

        let psi_small = psi(&small_shift, &reference, 10).unwrap();
        let psi_large = psi(&large_shift, &reference, 10).unwrap();

        assert!(
            psi_large > psi_small,
            "larger shift should produce larger PSI: small={psi_small}, large={psi_large}"
        );
    }

    #[test]
    fn test_psi_known_value() {
        // Hand-computed reference case with 2 bins.
        // current:   [0, 0, 0, 1, 1] → bins [0,0.5): 3/5=0.6, [0.5,1]: 2/5=0.4
        // reference: [0, 0, 1, 1, 1] → bins [0,0.5): 2/5=0.4, [0.5,1]: 3/5=0.6
        // PSI = (0.6-0.4)*ln(0.6/0.4) + (0.4-0.6)*ln(0.4/0.6)
        //     = 0.2*ln(1.5) + (-0.2)*ln(0.667)
        //     = 0.2*0.405465 + (-0.2)*(-0.405465) ≈ 0.16219
        let current = f64_array(&[0.0, 0.0, 0.0, 1.0, 1.0]);
        let reference = f64_array(&[0.0, 0.0, 1.0, 1.0, 1.0]);
        let result = psi(&current, &reference, 2).unwrap();
        let expected = 0.2 * (0.6f64 / 0.4).ln() + (-0.2) * (0.4f64 / 0.6).ln();
        assert!(
            (result - expected).abs() < 1e-10,
            "expected {expected}, got {result}"
        );
    }
}
