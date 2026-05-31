use arrow::array::ArrayRef;

use crate::error::Result;

/// Hellinger distance between two distributions (histogram-based).
///
/// Returns a value in [0, 1]. Matches scipy/numpy reference computation.
pub fn hellinger(current: &ArrayRef, reference: &ArrayRef, n_bins: usize) -> Result<f64> {
    let _ = (current, reference, n_bins);
    todo!("Phase 1: implement Hellinger via shared histogram builder")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hellinger_identical() {
        todo!()
    }

    #[test]
    fn test_hellinger_matches_reference_10k_samples() {
        todo!()
    }
}
