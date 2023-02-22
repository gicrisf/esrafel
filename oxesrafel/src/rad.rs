use pyo3::prelude::*;
use crate::par::Param;
use crate::nuc::Nucleus;
// use pyo3::types::PyList;

#[pyclass]
pub struct Radical {
    pub lwa: Param,
    pub lrtz: Param,
    pub amount: Param,
    pub dh1: Param,
    pub nucs: Vec<Nucleus>,
}

#[pymethods]
impl Radical {
    #[new]
    pub fn new(lwa: Param, lrtz: Param, amount: Param, dh1: Param) -> Self {
        let nucs = Vec::new();
        Self { lwa, lrtz, amount, dh1, nucs }
    }

    #[staticmethod]
    pub fn probe() -> Self {
        Self {
            lwa: Param::new(1.0, 0.0),
            lrtz: Param::new(50.0, 0.0),
            amount: Param::new(100.0, 0.0),
            dh1: Param::new(0.0, 0.0),
            nucs: Vec::new(),
        }
    }

    #[getter]
    pub fn get_lwa(&self) -> PyResult<Param> {
        Ok(self.lwa)
    }

    #[setter]
    pub fn set_lwa(&mut self, value: Param) -> PyResult<()> {
        self.lwa = value;
        Ok(())
    }

    #[getter]
    pub fn get_lrtz(&self) -> PyResult<Param> {
        Ok(self.lrtz)
    }

    #[setter]
    pub fn set_lrtz(&mut self, value: Param) -> PyResult<()> {
        self.lrtz = value;
        Ok(())
    }

    #[getter]
    pub fn get_amount(&self) -> PyResult<Param> {
        Ok(self.amount)
    }

    #[setter]
    pub fn set_amount(&mut self, value: Param) -> PyResult<()> {
        self.lwa = value;
        Ok(())
    }

    #[getter]
    pub fn get_dh1(&self) -> PyResult<Param> {
        Ok(self.dh1)
    }

    #[setter]
    pub fn set_dh1(&mut self, value: Param) -> PyResult<()> {
        self.dh1 = value;
        Ok(())
    }

    // TODO nucs getter

    pub fn push_nuc(&mut self, value: Nucleus) -> PyResult<()> {
        self.nucs.push(value);
        Ok(())
    }

    pub fn pop_nuc(&mut self) -> PyResult<()> {
        self.nucs.pop();
        Ok(())
    }
}
