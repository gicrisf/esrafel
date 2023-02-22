use pyo3::prelude::*;

#[derive(Clone, Debug, Copy)]
#[pyclass]
pub struct Param {
    pub val: f64,  // Value; starts with 0.0
    pub var: f64,  // Variation; starts with: 0.0
}

#[pymethods]
impl Param {
    #[new]
    pub fn new(val: f64, var: f64) -> Self {
        Param{ val, var }
    }

    #[getter]
    pub fn get_val(&self) -> PyResult<f64> {
        Ok(self.val)
    }

    #[setter]
    pub fn set_val(&mut self, value: f64) -> PyResult<()> {
        self.val = value;
        Ok(())
    }

    #[getter]
    pub fn get_var(&self) -> PyResult<f64> {
        Ok(self.var)
    }

    #[setter]
    pub fn set_var(&mut self, value: f64) -> PyResult<()> {
        self.var = value;
        Ok(())
    }

    pub fn randomize(&mut self) -> PyResult<()> {
        let temp = libesrafel::Param::set(self.val, self.var).randomize();
        self.val = temp.val;
        Ok(())
    }
}
