use arrow::array::ArrayRef;

use crate::error::Result;

/// First Wasserstein distance (Earth Mover's Distance) between two 1-D distributions.
///
/// Computed via sorted ECDFs; matches scipy.stats.wasserstein_distance.
pub fn wasserstein(current: &ArrayRef, reference: &ArrayRef) -> Result<f64> {
    let _ = (current, reference);
    todo!("Phase 1: implement Wasserstein via sorted arrays")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasserstein_identical() {
        todo!()
    }

    #[test]
    fn test_wasserstein_matches_scipy_10k_samples() {
        todo!()
    }
}
