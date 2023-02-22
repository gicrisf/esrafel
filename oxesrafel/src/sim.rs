use pyo3::prelude::*;
use crate::rad::Radical;

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
    pub fn new(sweep: f64, points: f64, rads: Vec<Radical>) -> Self {
        Self {
            rads,
            sweep,
            points,
        }
    }

    pub fn calc(&self) -> PyResult<Vec<f64>> {
        let rads = &self.rads.clone().into_iter().map(|r| rad_to_rs(&r)).collect();
        Ok(libesrafel::eprft::calcola(rads, self.sweep, self.points))
    }

    #[getter]
    pub fn get_rads(&self) -> PyResult<Vec<Radical>> {
        Ok(self.rads.clone())
    }

}
