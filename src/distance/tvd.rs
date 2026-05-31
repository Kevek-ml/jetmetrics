use arrow::array::ArrayRef;

use crate::error::Result;

/// Total Variation Distance between two distributions (histogram-based).
///
/// Returns a value in [0, 1]. Matches reference implementation.
pub fn tvd(current: &ArrayRef, reference: &ArrayRef, n_bins: usize) -> Result<f64> {
    let _ = (current, reference, n_bins);
    todo!("Phase 1: implement TVD via shared histogram builder")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tvd_identical() {
        todo!()
    }

    #[test]
    fn test_tvd_matches_reference_10k_samples() {
        todo!()
    }
}
