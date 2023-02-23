mod par;
mod nuc;
mod rad;
mod sim;
mod iof;

use pyo3::prelude::*;
use crate::par::Param;
use crate::nuc::Nucleus;
use crate::rad::Radical;
use crate::sim::Simulator;
use crate::iof::get_from_ascii;
use crate::iof::ascii_import;
use crate::iof::ascii_to_json;
use crate::iof::get_from_sim;
use crate::iof::sim_as_json;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxesrafel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_ascii, m)?)?;
    m.add_function(wrap_pyfunction!(ascii_import, m)?)?;
    m.add_function(wrap_pyfunction!(ascii_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_sim, m)?)?;
    m.add_function(wrap_pyfunction!(sim_as_json, m)?)?;
    m.add_class::<Param>()?;
    m.add_class::<Nucleus>()?;
    m.add_class::<Radical>()?;
    m.add_class::<Simulator>()?;
    Ok(())
}
