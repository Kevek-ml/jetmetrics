use arrow::array::ArrayRef;

use crate::error::Result;

/// Empirical CDF as sorted values + cumulative probabilities.
///
/// Used by the KS test and Wasserstein distance.
pub struct Ecdf {
    /// Sorted sample values.
    pub values: Vec<f64>,
    /// Cumulative probability at each value (i+1 / n).
    pub probabilities: Vec<f64>,
}

impl Ecdf {
    /// Build an ECDF from an Arrow float array.
    pub fn build(array: &ArrayRef) -> Result<Self> {
        let _ = array;
        todo!("Phase 2: sort via Arrow sort kernel, compute cumulative probs")
    }

    /// Evaluate the ECDF at a given x (returns F(x)).
    pub fn evaluate(&self, x: f64) -> f64 {
        let _ = x;
        todo!()
    }
}
