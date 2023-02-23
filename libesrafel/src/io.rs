use crate::{Radical, Nucleus};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Default, Serialize, Deserialize)]
pub struct Spectrum {
    idx: Vec<usize>,
    fld: Vec<f64>,
    int: Vec<f64>
}

impl Spectrum {
    pub fn into_tuple(&self) -> (Vec<usize>, Vec<f64>, Vec<f64>) {
        (self.idx.clone(), self.fld.clone(), self.int.clone())
    }

    pub fn get_int(&self) -> Vec<f64> {
        self.int.clone()
    }

    // Serialize as JSON
    pub fn into_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

// TODO DELETE
// Wrapper around ascii_import function
// (temp leaving it for retrocompatibility with Esrafel GTK4)
pub fn get_from_ascii(content: &str) -> Vec<f64> {
    ascii_import(content).get_int()
}

// TODO make spectrum method from_ascii
pub fn ascii_import(content: &str) -> Spectrum {
    let mut imp = Spectrum {
        idx: Vec::new(),
        fld: Vec::new(),
        int: Vec::new(),
    };

    // TODO refactor with map
    // let result = content.lines().map(|line| {...}
     for line in content.lines() {
        // Looks like trim is implicit in split_whitespace
        let line = line.trim();  // like Python's strip;
        // TODO split_ascii_whitespace?
        let cols = line.split_whitespace(); // like Python's split;

         // Questo è già un controllo supplementare,
         // Se arrivi alla quarta colonna abort
         // Se non arrivi alla seconda abort
         // Se non arrivi alla prima abort.
         // Ecc.
         // Inutile clonare a monte
        if cols.clone().count() == 3 {  // Can I replace the clone() solution?
            for (idx, col) in cols.enumerate() {
                match idx {
                    0 => Some(imp.idx.push(col.parse().unwrap())),
                    1 => Some(imp.fld.push(col.parse().unwrap())),
                    2 => Some(imp.int.push(col.parse().unwrap())),
                    _ => None,
                };
            }
        }
    };

    imp
}

#[derive(Default, Serialize, Deserialize)]
pub struct SimulationState {
    points: i32,
    sweep: i32,
    rads: Vec<Radical>
}

// TODO make this a method
pub fn get_from_sim(data: &str) -> SimulationState {
    let mut lines = data.lines();
    let mut rads = Vec::new();

    let how_many_rads: i32 = lines.next().unwrap().trim().parse().expect("Cannot read how many Radicals");
    let points: i32 = lines.next().unwrap().trim().parse().unwrap();
    let sweep: i32 = lines.next().unwrap().trim().parse().unwrap();

    // println!("rads: {}, points: {}, sweep: {}", how_many_rads, points, sweep);

    for _ in 0..how_many_rads {
        let amount: f64 = lines.next().unwrap().trim().parse().unwrap();
        let dh1: f64 = lines.next().unwrap().trim().parse().unwrap();
        let lwa: f64 = lines.next().unwrap().trim().parse().unwrap();
        let lrtz: f64 = lines.next().unwrap().trim().parse().unwrap();

        // println!("New radical with amount {}, center {}, lwa {}, lrtz {}", amount, center, lw, lrtz);

        let how_many_const: i32 = lines.next().unwrap().trim().parse().unwrap();

        let mut nucs = Vec::new();
        for _ in 0..how_many_const {
            let eqs: i32 = lines.next().unwrap().trim().parse().unwrap();
            let spin: f64 = lines.next().unwrap().trim().parse().unwrap();
            let hpf: f64 = lines.next().unwrap().trim().parse().unwrap();

            // println!("New nuc with eqs {}, spin {}, hpf {}", nuclei, spin, hpf);
            nucs.push(Nucleus::set(spin, hpf, eqs as f64));
        }  // for nuc in nucs

        rads.push(Radical::set(lwa, lrtz, amount, dh1, nucs));
    }  // for rad in rads

    SimulationState {
        points,
        sweep,
        rads,
    }
}

impl SimulationState {
    pub fn into_tuple(&self) -> (i32, i32, Vec<Radical>) {
        (self.points, self.sweep, self.rads.clone())
    }

    pub fn get_rads(&self) -> Vec<Radical> {
        self.rads.clone()
    }

    pub fn into_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}
