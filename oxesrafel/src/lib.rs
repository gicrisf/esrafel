use pyo3::prelude::*;
// use pyo3::types::PyList;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[derive(Clone, Debug, Copy)]
#[pyclass]
struct Param {
    pub val: f64,  // Value; starts with 0.0
    pub var: f64,  // Variation; starts with: 0.0
}

#[pymethods]
impl Param {
    #[new]
    fn new(val: f64, var: f64) -> Self {
        Param{ val, var }
    }

    #[getter]
    fn get_val(&self) -> PyResult<f64> {
        Ok(self.val)
    }

    #[setter]
    fn set_val(&mut self, value: f64) -> PyResult<()> {
        self.val = value;
        Ok(())
    }

    #[getter]
    fn get_var(&self) -> PyResult<f64> {
        Ok(self.var)
    }

    #[setter]
    fn set_var(&mut self, value: f64) -> PyResult<()> {
        self.var = value;
        Ok(())
    }

    fn randomize(&mut self) -> PyResult<()> {
        let temp = libesrafel::Param::set(self.val, self.var).randomize();
        self.val = temp.val;
        Ok(())
    }
}

#[derive(Clone)]
#[pyclass]
struct Nucleus {
    pub spin: Param,
    pub hpf: Param, 
    pub eqs: Param,
}

#[pymethods]
impl Nucleus {
    #[new]
    fn new(spin: Param, hpf: Param, eqs: Param) -> Self {
        Nucleus { spin, hpf, eqs }
    }

    #[staticmethod]
    fn probe() -> Self {
        Nucleus::new(Param::new(1.0, 0.0), Param::new(1.0, 0.0), Param::new(1.0, 0.0))
    }

    #[getter]
    fn get_spin(&self) -> PyResult<Param> {
        Ok(self.spin)
    }

    #[setter]
    fn set_spin(&mut self, value: Param) -> PyResult<()> {
        self.spin = value;
        Ok(())
    }

    #[getter]
    fn get_hpf(&self) -> PyResult<Param> {
        Ok(self.hpf)
    }

    #[setter]
    fn set_hpf(&mut self, value: Param) -> PyResult<()> {
        self.hpf = value;
        Ok(())
    }

    #[getter]
    fn get_eqs(&self) -> PyResult<Param> {
        Ok(self.eqs)
    }

    #[setter]
    fn set_eqs(&mut self, value: Param) -> PyResult<()> {
        self.eqs = value;
        Ok(())
    }
}

#[pyclass]
struct Radical {
    pub lwa: Param,
    pub lrtz: Param,
    pub amount: Param,
    pub dh1: Param,
    pub nucs: Vec<Nucleus>,
}

#[pymethods]
impl Radical {
    #[new]
    fn new(lwa: Param, lrtz: Param, amount: Param, dh1: Param) -> Self {
        let nucs = Vec::new();
        Self { lwa, lrtz, amount, dh1, nucs }
    }

    #[staticmethod]
    fn probe() -> Self {
        Self {
            lwa: Param::new(1.0, 0.0),
            lrtz: Param::new(50.0, 0.0),
            amount: Param::new(100.0, 0.0),
            dh1: Param::new(0.0, 0.0),
            nucs: Vec::new(),
        }
    }

    #[getter]
    fn get_lwa(&self) -> PyResult<Param> {
        Ok(self.lwa)
    }

    #[setter]
    fn set_lwa(&mut self, value: Param) -> PyResult<()> {
        self.lwa = value;
        Ok(())
    }

    #[getter]
    fn get_lrtz(&self) -> PyResult<Param> {
        Ok(self.lrtz)
    }

    #[setter]
    fn set_lrtz(&mut self, value: Param) -> PyResult<()> {
        self.lrtz = value;
        Ok(())
    }

    #[getter]
    fn get_amount(&self) -> PyResult<Param> {
        Ok(self.amount)
    }

    #[setter]
    fn set_amount(&mut self, value: Param) -> PyResult<()> {
        self.lwa = value;
        Ok(())
    }

    #[getter]
    fn get_dh1(&self) -> PyResult<Param> {
        Ok(self.dh1)
    }

    #[setter]
    fn set_dh1(&mut self, value: Param) -> PyResult<()> {
        self.dh1 = value;
        Ok(())
    }

    // TODO nucs getter

    fn push_nuc(&mut self, value: Nucleus) -> PyResult<()> {
        self.nucs.push(value);
        Ok(())
    }

    fn pop_nuc(&mut self) -> PyResult<()> {
        self.nucs.pop();
        Ok(())
    }
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

/// A Python module implemented in Rust.
#[pymodule]
fn oxesrafel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_ascii, m)?)?;
    m.add_function(wrap_pyfunction!(calcola, m)?)?;
    m.add_class::<Param>()?;
    m.add_class::<Nucleus>()?;
    m.add_class::<Radical>()?;
    Ok(())
}
