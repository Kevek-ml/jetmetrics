use arrow::record_batch::RecordBatch;

use crate::error::Result;

/// Number of rows in a RecordBatch.
pub fn row_count(batch: &RecordBatch) -> usize {
    batch.num_rows()
}

/// Number of columns in a RecordBatch.
pub fn column_count(batch: &RecordBatch) -> usize {
    batch.num_columns()
}

/// Null count per column. Returns a Vec of (column_name, null_count) pairs.
pub fn null_counts(batch: &RecordBatch) -> Result<Vec<(String, usize)>> {
    let _ = batch;
    todo!("Phase 3: use Arrow null_count kernel per column")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_count() {
        todo!()
    }

    #[test]
    fn test_null_counts_per_column() {
        todo!()
    }
}
