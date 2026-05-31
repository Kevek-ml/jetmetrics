use arrow::array::ArrayRef;

use crate::error::Result;

/// Result of a two-sample KS test.
pub struct KsTestResult {
    pub statistic: f64,
    pub p_value: f64,
}

/// Two-sample Kolmogorov-Smirnov test.
///
/// p-value matches scipy.stats.ks_2samp to 8 decimal places.
pub fn ks_2samp(current: &ArrayRef, reference: &ArrayRef) -> Result<KsTestResult> {
    let _ = (current, reference);
    todo!("Phase 2: implement KS test via ECDF + special functions")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ks_identical_distributions() {
        todo!()
    }

    #[test]
    fn test_ks_p_value_matches_scipy_8_decimal_places() {
        todo!()
    }
}
