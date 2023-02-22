// IO functions
use pyo3::prelude::*;

// Get vector from ASCII input
#[pyfunction]
pub fn get_from_ascii(content: &str) -> PyResult<Vec<f64>> {
    Ok(libesrafel::io::get_from_ascii(content))
}
