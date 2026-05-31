use arrow::array::ArrayRef;

use crate::error::Result;

/// Jensen-Shannon divergence between two distributions (histogram-based).
///
/// Returns a value in [0, 1] (square-root form). Matches scipy.spatial.distance.jensenshannon.
pub fn js_divergence(current: &ArrayRef, reference: &ArrayRef, n_bins: usize) -> Result<f64> {
    let _ = (current, reference, n_bins);
    todo!("Phase 1: implement JS divergence via shared histogram builder")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_identical() {
        todo!()
    }

    #[test]
    fn test_js_matches_scipy_10k_samples() {
        todo!()
    }
}
