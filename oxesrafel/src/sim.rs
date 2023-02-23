use pyo3::prelude::*;
use crate::nuc::Nucleus;
use crate::rad::Radical;
use crate::par::Param;

#[pyclass]
pub struct Simulator {
    pub rads: Vec<Radical>,
    pub sweep: f64,
    pub points: f64,
}

// TODO impl for nucs and rads (no py methods!)
fn nuc_to_rs(nuc: &Nucleus) -> libesrafel::Nucleus {
    libesrafel::Nucleus {
        spin: libesrafel::Param::set(nuc.spin.val, nuc.spin.var),
        hpf: libesrafel::Param::set(nuc.hpf.val, nuc.hpf.var),
        eqs: libesrafel::Param::set(nuc.eqs.val, nuc.eqs.var),
    }
}

fn nuc_to_py(nuc: &libesrafel::Nucleus) -> Nucleus {
    Nucleus {
        spin: Param::new(nuc.spin.val, nuc.spin.var),
        hpf: Param::new(nuc.hpf.val, nuc.hpf.var),
        eqs: Param::new(nuc.eqs.val, nuc.eqs.var),
    }
}

fn rad_to_rs(rad: &Radical) -> libesrafel::Radical {
    let nucs = rad.nucs.clone().into_iter().map(|n| nuc_to_rs(&n)).collect();

    libesrafel::Radical {
        lwa: libesrafel::Param::set(rad.lwa.val, rad.lwa.var),
        lrtz: libesrafel::Param::set(rad.lrtz.val, rad.lrtz.var),
        amount: libesrafel::Param::set(rad.amount.val, rad.amount.var),
        dh1: libesrafel::Param::set(rad.dh1.val, rad.dh1.var),
        nucs,
    }
}

pub fn rad_to_py(rad: &libesrafel::Radical) -> Radical {
    let nucs = rad.nucs.clone().into_iter().map(|n| nuc_to_py(&n)).collect();

    Radical {
        lwa: Param::new(rad.lwa.val, rad.lwa.var),
        lrtz: Param::new(rad.lrtz.val, rad.lrtz.var),
        amount: Param::new(rad.amount.val, rad.amount.var),
        dh1: Param::new(rad.dh1.val, rad.dh1.var),
        nucs,
    }
}

#[pymethods]
impl Simulator {
    #[new]
    pub fn new(sweep: f64, points: f64, rads: Vec<Radical>) -> Self {
        Self { sweep, points, rads }
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
