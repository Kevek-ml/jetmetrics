use arrow::array::ArrayRef;

use crate::error::Result;

/// Result of an independent-samples t-test.
pub struct TTestResult {
    pub statistic: f64,
    pub p_value: f64,
}

/// Welch's independent two-sample t-test (unequal variances).
///
/// p-value matches scipy.stats.ttest_ind(equal_var=False) to 8 decimal places.
pub fn ttest_ind(current: &ArrayRef, reference: &ArrayRef) -> Result<TTestResult> {
    let _ = (current, reference);
    todo!("Phase 2: implement Welch's t-test via Welford + Beta CDF")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ttest_identical_distributions() {
        todo!()
    }

    #[test]
    fn test_ttest_p_value_matches_scipy_8_decimal_places() {
        todo!()
    }
}
