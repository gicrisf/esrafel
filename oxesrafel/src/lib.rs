mod par;
mod nuc;
mod rad;

use pyo3::prelude::*;
use crate::par::Param;
use crate::nuc::Nucleus;
use crate::rad::Radical;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Get vector from ASCII input
#[pyfunction]
fn get_from_ascii(content: &str) -> PyResult<Vec<f64>> {
    Ok(libesrafel::io::get_from_ascii(content))
}

#[pyfunction]
fn calcola() -> PyResult<Vec<f64>> {
    let mut rads = Vec::new();
    let sweep = 100.0;
    let points = 1024.0;
    rads.push(libesrafel::Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new()));
    Ok(libesrafel::eprft::calcola(&rads, sweep, points))
}

#[pyclass]
pub struct Simulator {
    pub rads: Vec<Radical>,
    pub sweep: f64,
    pub points: f64,
}

fn rad_to_rs(rad: &Radical) -> libesrafel::Radical {
    libesrafel::Radical {
        lwa: libesrafel::Param::set(rad.lwa.val, rad.lwa.var),
        lrtz: libesrafel::Param::set(rad.lrtz.val, rad.lrtz.var),
        amount: libesrafel::Param::set(rad.amount.val, rad.amount.var),
        dh1: libesrafel::Param::set(rad.dh1.val, rad.dh1.var),
        nucs: Vec::new(),
    }
}

#[pymethods]
impl Simulator {
    #[new]
    pub fn new(sweep: f64, points: f64) -> Self {
        Self {
            rads: Vec::new(),
            sweep,
            points,
        }
    }

    pub fn calc(&self) -> PyResult<Vec<f64>> {
        let mut rads = Vec::new();
        for rad in &self.rads {
            rads.push(rad_to_rs(rad))
        };

        // debug
        rads.push(libesrafel::Radical::_probe());
        Ok(libesrafel::eprft::calcola(&rads, self.sweep, self.points))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxesrafel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_ascii, m)?)?;
    m.add_function(wrap_pyfunction!(calcola, m)?)?;
    m.add_class::<Param>()?;
    m.add_class::<Nucleus>()?;
    m.add_class::<Radical>()?;
    m.add_class::<Simulator>()?;
    Ok(())
}
