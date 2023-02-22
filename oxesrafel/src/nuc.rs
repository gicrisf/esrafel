use pyo3::prelude::*;
use crate::par::Param;

#[derive(Clone)]
#[pyclass]
pub struct Nucleus {
    pub spin: Param,
    pub hpf: Param,
    pub eqs: Param,
}

#[pymethods]
impl Nucleus {
    #[new]
    pub fn new(spin: Param, hpf: Param, eqs: Param) -> Self {
        Nucleus { spin, hpf, eqs }
    }

    #[staticmethod]
    pub fn probe() -> Self {
        Nucleus::new(Param::new(1.0, 0.0), Param::new(1.0, 0.0), Param::new(1.0, 0.0))
    }

    #[getter]
    pub fn get_spin(&self) -> PyResult<Param> {
        Ok(self.spin)
    }

    #[setter]
    pub fn set_spin(&mut self, value: Param) -> PyResult<()> {
        self.spin = value;
        Ok(())
    }

    #[getter]
    pub fn get_hpf(&self) -> PyResult<Param> {
        Ok(self.hpf)
    }

    #[setter]
    pub fn set_hpf(&mut self, value: Param) -> PyResult<()> {
        self.hpf = value;
        Ok(())
    }

    #[getter]
    pub fn get_eqs(&self) -> PyResult<Param> {
        Ok(self.eqs)
    }

    #[setter]
    pub fn set_eqs(&mut self, value: Param) -> PyResult<()> {
        self.eqs = value;
        Ok(())
    }
}
