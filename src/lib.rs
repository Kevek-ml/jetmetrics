use pyo3::prelude::*;

/// jetmetrics — Arrow-native statistical metrics for ML monitoring
#[pymodule]
fn _core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add(
        "__doc__",
        "Arrow-native, Rust-based statistical metrics for ML monitoring",
    )?;
    Ok(())
}
