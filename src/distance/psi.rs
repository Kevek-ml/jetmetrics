use arrow::array::ArrayRef;

use crate::error::Result;

/// Population Stability Index between `current` and `reference` distributions.
///
/// Supports both continuous (histogram-binned) and discrete inputs.
/// Matches scipy histogram-based PSI to bitwise precision.
pub fn psi(current: &ArrayRef, reference: &ArrayRef, n_bins: usize) -> Result<f64> {
    let _ = (current, reference, n_bins);
    todo!("Phase 1: implement PSI via shared histogram builder")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psi_identical_distributions() {
        // PSI should be 0 when current == reference
        todo!()
    }

    #[test]
    fn test_psi_matches_scipy_10k_samples() {
        todo!()
    }
}
