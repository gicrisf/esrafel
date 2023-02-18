pub mod eprft;
pub mod io;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Param {
    pub val: f64,  // Value; starts with 0.0
    pub var: f64,  // Variation; starts with: 0.0
}

impl Param {
    pub fn set(val: f64, var: f64) -> Param {
        Param { val, var, }
    }

    pub fn randomize(&self) -> Param {
        if self.var != 0.0 {
            let mut rng = thread_rng();
            let random: f64 = rng.gen();  // random number in range [0, 1)
            let rnd = 2.0*random-1.0;
            let new_val = self.val + rnd * self.var;
            return Param { val: new_val, var: self.var }
        } else {
            return Param { val: self.val, var: self.var }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nucleus {
    pub spin: Param,  // Nuclear spin;
    pub hpf: Param,  // Hyperfine constant;
    pub eqs: Param,  // Equivalent nucleus; Should be u8!
}

impl Nucleus {
    pub fn set(spin: f64, hpf: f64, eqs: f64) -> Nucleus {
        Nucleus {
            spin: Param::set(spin, 0.0),
            hpf: Param::set(hpf, 0.0),
            eqs: Param::set(eqs, 0.0),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Radical {
    pub lwa: Param,  // Line width A
    // pub lwb: Param,
    // pub lwc: Param,
    pub lrtz: Param,  // Lorentzian linewidth parameter (%)
    pub amount: Param,  // Relative amount
    pub dh1: Param,
    pub nucs: Vec<Nucleus>,
}

impl Radical {
    pub fn set(lwa: f64, lrtz: f64, amount: f64, dh1: f64, nucs: Vec<Nucleus>) -> Self {
        Self {
            lwa: Param::set(lwa, 0.0),
            lrtz: Param::set(lrtz, 0.0),
            amount: Param::set(amount, 0.0),
            dh1: Param::set(dh1, 0.0),
            nucs,
        }
    }

    // Radical without nuclei and standard parameters;
    pub fn _electron() -> Radical {
        Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new())
    }

    // Nitroxide-like test radical
    pub fn _probe() -> Radical {
        let mut rad = Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new());
        rad.nucs.push(Nucleus::set(1.0, 14.0, 1.0));
        rad
    }

    pub fn var_probe() -> Radical {
        let mut rad = Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new());
        rad.nucs.push(Nucleus::set(1.0, 19.0, 1.0));
        rad.nucs[0].hpf.var = 1.0;
        rad.lwa.var = 0.1;
        rad.dh1.var = 0.1;
        rad.lrtz.var = 0.1;
        rad.amount.var = 0.1;
        rad
    } // var probe
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn load_spectrum() {
        // Extract from real data
        let input_text = concat!(
            "index            Field [G]         Intensity []\n",
            "1              3262.75      4600.7724609375\n",
            "2     3262.81842619746      5483.7724609375\n",
            "3     3262.88685239492      1550.7724609375\n",
            "4     3262.95527859238       -22.2275390625\n"
        );

        let result = io::get_from_ascii(input_text);

        // Test against a Vec<f64>
        assert_eq!(result, vec![4600.7724609375, 5483.7724609375, 1550.7724609375, -22.2275390625])
    }
}
