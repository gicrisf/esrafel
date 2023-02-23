use pyo3::prelude::*;
use crate::sim::rad_to_py;
use crate::rad::Radical;

// Get vector from ASCII input
#[pyfunction]
pub fn get_from_ascii(content: &str) -> PyResult<Vec<f64>> {
    Ok(libesrafel::io::get_from_ascii(content))
}

#[pyfunction]
pub fn ascii_import(content: &str) -> PyResult<(Vec<f64>, Vec<f64>)> {
    Ok(libesrafel::io::ascii_import(content))
}

#[pyfunction]
pub fn get_from_sim(content: &str) -> PyResult<(i32, i32, Vec<Radical>)> {
    let (points, sweep, rads) = libesrafel::io::get_from_sim(content);
    let rads = rads.clone().into_iter().map(|r| rad_to_py(&r)).collect();
    Ok((points, sweep, rads))
}
