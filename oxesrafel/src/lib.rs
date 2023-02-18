use pyo3::prelude::*;
use libesrafel::Radical;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Get vector from ASCII input
#[pyfunction]
fn get_from_ascii(content: &str) -> PyResult<Vec<f64>> {
    Ok(libesrafel::get_from_ascii(content))
}

#[pyfunction]
fn calcola() -> PyResult<Vec<f64>> {
    let mut rads = Vec::new();
    let sweep = 100.0;
    let points = 1024.0;
    rads.push(Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new()));
    Ok(libesrafel::cmd99::calcola(&rads, sweep, points))
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxesrafel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_ascii, m)?)?;
    m.add_function(wrap_pyfunction!(calcola, m)?)?;
    Ok(())
}
