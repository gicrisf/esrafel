use pyo3::prelude::*;
use crate::sim::rad_to_py;
use crate::rad::Radical;

// Get vector from ASCII input
#[pyfunction]
pub fn get_from_ascii(content: &str) -> PyResult<Vec<f64>> {
    Ok(libesrafel::io::get_from_ascii(content))
}

#[pyfunction]
pub fn ascii_import(content: &str) -> PyResult<(Vec<usize>, Vec<f64>, Vec<f64>)> {
    Ok(libesrafel::io::ascii_import(content).into_tuple())
}

#[pyfunction]
pub fn ascii_to_json(content: &str) -> PyResult<String> {
    Ok(libesrafel::io::ascii_import(content).into_json().unwrap())
}

#[pyfunction]
pub fn get_from_sim(content: &str) -> PyResult<(i32, i32, Vec<Radical>)> {
    let (points, sweep, rads) = libesrafel::io::get_from_sim(content).into_tuple();
    let rads = rads.into_iter().map(|r| rad_to_py(&r)).collect();
    Ok((points, sweep, rads))
}

#[pyfunction]
pub fn sim_as_json(content: &str) -> PyResult<String> {
    Ok(libesrafel::io::get_from_sim(content).into_json().unwrap())
}
