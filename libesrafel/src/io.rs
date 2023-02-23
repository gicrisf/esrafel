use crate::{Radical, Nucleus};

// Read spectrum from an ASCII source
// TODO DELETE (temp leaving it for retrocompatibility with Esrafel GTK4)
pub fn get_from_ascii(content: &str) -> Vec<f64> {
    // let mut fld = Vec::new();  // field
    let mut int = Vec::new();  // intensity

    for line in content.lines() {
        let line = line.trim();  // like Python's strip;
        // TODO split_ascii_whitespace?
        let cols = line.split_whitespace(); // like Python's split;

        if cols.clone().count() == 3 {  // Can I replace the clone() solution?
            for (idx, col) in cols.enumerate() {
                match idx {
                    // 1 => Some(fld.push(col.parse().unwrap())),
                    2 => Some(int.push(col.parse().unwrap())),
                    _ => None,
                };
            }
        }
    };

    int
}

// TODO make it better, the functional way
pub fn ascii_import(content: &str) -> (Vec<f64>, Vec<f64>) {
    let mut fld: Vec<f64> = Vec::new();  // field
    let mut int: Vec<f64> = Vec::new();  // intensity

    // let cols = content.lines().map(|l| l.trim().split_whitespace());

    for line in content.lines() {
        let line = line.trim();  // like Python's strip;
        // TODO split_ascii_whitespace?
        let cols = line.split_whitespace(); // like Python's split;

        if cols.clone().count() == 3 {  // Can I replace the clone() solution?
            for (idx, col) in cols.enumerate() {
                match idx {
                    1 => {
                        fld.push(col.parse().unwrap());
                        Some(())
                    },
                    2 => {
                        int.push(col.parse().unwrap());
                        Some(())
                    },
                    _ => None,
                };
            }
        }
    };

    (fld, int)
}

pub fn get_from_sim(data: &str) -> (i32, i32, Vec<Radical>) {
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

    (points, sweep, rads)
}
