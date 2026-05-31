use pyo3::prelude::*;
use pyo3::Bound;

pub mod accumulator;
pub mod dataset;
pub mod distance;
pub mod error;
pub mod performance;
pub mod stat_tests;
pub mod util;

/// jetmetrics — Arrow-native statistical metrics for ML monitoring
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add(
        "__doc__",
        "Arrow-native, Rust-based statistical metrics for ML monitoring",
    )?;
    Ok(())
}
