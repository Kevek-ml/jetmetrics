use arrow::record_batch::RecordBatch;

/// Core trait for streaming metric computation.
///
/// Designed for upgrade path: v0.1 (in-memory) → v0.2 (RecordBatchReader) → v0.3 (DataFusion UDAF).
pub trait Accumulator: Sized {
    /// Absorb one Arrow chunk.
    fn update(&mut self, batch: &RecordBatch) -> crate::error::Result<()>;

    /// Merge another accumulator of the same type (enables parallel execution).
    fn merge(&mut self, other: Self) -> crate::error::Result<()>;

    /// Produce the final metric value.
    fn finalize(&self) -> crate::error::Result<f64>;
}
