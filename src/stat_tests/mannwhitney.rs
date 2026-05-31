use arrow::array::ArrayRef;

use crate::error::Result;

/// Result of a Mann-Whitney U test.
pub struct MannWhitneyResult {
    pub statistic: f64,
    pub p_value: f64,
}

/// Two-sample Mann-Whitney U test (Wilcoxon rank-sum test).
///
/// p-value matches scipy.stats.mannwhitneyu to 8 decimal places.
/// Handles ties via normal approximation with tie correction.
pub fn mannwhitneyu(current: &ArrayRef, reference: &ArrayRef) -> Result<MannWhitneyResult> {
    let _ = (current, reference);
    todo!("Phase 2: implement Mann-Whitney via rank computation + normal approximation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mannwhitney_identical_distributions() {
        todo!()
    }

    #[test]
    fn test_mannwhitney_p_value_matches_scipy_8_decimal_places() {
        todo!()
    }

    #[test]
    fn test_mannwhitney_handles_ties() {
        todo!()
    }
}
