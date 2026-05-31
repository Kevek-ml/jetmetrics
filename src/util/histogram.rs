use arrow::array::ArrayRef;

use crate::error::Result;

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
    /// Build a histogram with `n_bins` equal-width bins.
    ///
    /// `min` and `max` are pre-computed over the union of current + reference
    /// so callers can reuse the same edges for both arrays.
    pub fn build(array: &ArrayRef, n_bins: usize, min: f64, max: f64) -> Result<Self> {
        let _ = (array, n_bins, min, max);
        todo!("Phase 1: SIMD-optimized binning via Arrow cast + compute kernels")
    }

    /// Normalize counts to a probability distribution (sums to 1).
    pub fn normalize(&self) -> Vec<f64> {
        let total: u64 = self.counts.iter().sum();
        self.counts
            .iter()
            .map(|&c| c as f64 / total as f64)
            .collect()
    }
}

/// Compute shared bin edges from the union range of two arrays.
pub fn shared_edges(a: &ArrayRef, b: &ArrayRef, n_bins: usize) -> Result<(f64, f64)> {
    let _ = (a, b, n_bins);
    todo!("Phase 1: use Arrow min/max compute kernels")
}
