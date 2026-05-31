use arrow::array::ArrayRef;

use crate::error::Result;

/// Mean Absolute Error. Matches sklearn.metrics.mean_absolute_error.
pub fn mae(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Mean Squared Error. Matches sklearn.metrics.mean_squared_error.
pub fn mse(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Root Mean Squared Error. Matches sqrt(sklearn.metrics.mean_squared_error).
pub fn rmse(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Mean Absolute Percentage Error. Matches sklearn.metrics.mean_absolute_percentage_error.
pub fn mape(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

/// Coefficient of determination (R²). Matches sklearn.metrics.r2_score.
pub fn r2(actual: &ArrayRef, predicted: &ArrayRef) -> Result<f64> {
    let _ = (actual, predicted);
    todo!("Phase 3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mae_matches_sklearn() {
        todo!()
    }

    #[test]
    fn test_mse_matches_sklearn() {
        todo!()
    }

    #[test]
    fn test_rmse_matches_sklearn() {
        todo!()
    }
}
