use arrow::array::ArrayRef;

use crate::error::Result;

/// Fraction of correctly classified samples. Matches sklearn.metrics.accuracy_score.
pub fn accuracy(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// F1 score (macro-averaged for multiclass). Matches sklearn.metrics.f1_score.
pub fn f1(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Precision. Matches sklearn.metrics.precision_score.
pub fn precision(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Recall. Matches sklearn.metrics.recall_score.
pub fn recall(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Area under the ROC curve. Matches sklearn.metrics.roc_auc_score (bitwise).
pub fn auc_roc(actual: &ArrayRef, scores: &ArrayRef) -> Result<f64> {
    let _ = (actual, scores);
    todo!("Phase 3: implement via sort + trapezoid rule")
}

/// Area under the Precision-Recall curve. Matches sklearn.metrics.average_precision_score.
pub fn auc_pr(actual: &ArrayRef, scores: &ArrayRef) -> Result<f64> {
    let _ = (actual, scores);
    todo!("Phase 3")
}

/// Log loss (cross-entropy). Matches sklearn.metrics.log_loss.
pub fn log_loss(actual: &ArrayRef, probabilities: &ArrayRef) -> Result<f64> {
    let _ = (actual, probabilities);
    todo!("Phase 3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy_all_correct() {
        todo!()
    }

    #[test]
    fn test_auc_roc_matches_sklearn_bitwise() {
        todo!()
    }
}
