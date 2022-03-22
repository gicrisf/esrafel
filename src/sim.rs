// TODO use better var names
// Temporarily maintaining legacy name to make easier comparison

pub struct Param {
    pub val: f64,  // Value; starts with 0.0
    pub var: f64,  // Variation; starts with: 0.0
}

impl Param {
    pub fn set(val: f64, var: f64) -> Param {
        Param { val, var, }
    }
}

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
    pub fn electron() -> Radical {
        Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new())
    }

    // Nitroxide-like test radical
    pub fn probe() -> Radical {
        let mut rad = Radical::set(0.5, 100.0, 100.0, 0.0, Vec::new());
        rad.nucs.push(Nucleus::set(1.0, 14.0, 1.0));
        rad
    }
}

// Calculate teorical spectra
// pub fn calcola(rads: Vec<Radical>, sweep: f64, points: f64, sigma: f64) -> Vec<f64> {
pub fn calcola() -> Vec<f64> {
    let sweep: f64 = 100.0;
    let points: f64 = 1024.0;
    let sigma: f64 = 1E+20;
    let iters: usize = 0;
    let mut rads = Vec::new();
    rads.push(Radical::electron());
    // let sweep get from model
    // let sweep = self.sweep.lock().unwrap();

    let incrgauss = sweep/(points -1.0);
    let mut lno = vec![0.0; points as usize];
    let mut newteor = vec![0.0; points as usize];

    // Stickspectrum
    for rad in rads {
        let mut totale = 1.0;  // Total intensity
        let mut pf = 1.0;  // Max intensity point value
        let mut pcostanti: Vec<f64> = Vec::new();
        let mut spini: Vec<f64> = Vec::new();

        for nuc in &rad.nucs {
            let pcostante = nuc.hpf.val/incrgauss as f64;
            pcostanti.push(pcostante);
            spini.push(2.0*nuc.spin.val);
        }

        let mut pa = 1.0;  // peak area?
        for (i, nuc) in rad.nucs.iter().enumerate() {
            pa = pa + pcostanti[i] * spini[i] * nuc.eqs.val;
        }
        if pa < points { pa = points; }

        let mut intensity = vec![0.0; pa as usize];
        intensity[1] = 1.0;  // TODO: check

        for (i, nuc) in rad.nucs.iter().enumerate() {
            let mut eq = 1;
            while eq <= nuc.eqs.val as usize {
                let mut indice1 = pf as usize;
                while indice1 > 0 {
                    if intensity[indice1] != 0.0 {
                        let mut i2 = 1.0;
                        while i2 <= (2.0*nuc.spin.val) {
                            let new = indice1 as f64 + i2 * pcostanti[i];
                            intensity[new as usize]+=intensity[indice1];
                            totale+=intensity[indice1];

                            i2+=1.0;

                            if new > pf {
                                pf = new;
                            }
                        }  // while i2...

                    }  // if intensity[indice1]...

                    indice1 = indice1 - 1; // Decrement
                }  // while indice1...

                eq+=1;
            }  // for(eq=1;eq<=nucleis[l][i];i1++)
        }  // for nuc in rad.nucs

        let shift: isize = ((points as isize)-(pf as isize))/2;
        let shift_abs: usize = shift.abs() as usize;  // Eraseme

        if shift < 0 {
            let mut point = 1;
            while point < points as usize {
                intensity[point] = intensity[point-shift_abs];
                intensity[point-shift_abs] = 0.0;

                point+=1;  // Increment
            }  // for(i=1;i<=punti;i++)
        } else if shift > 0 {
            let mut point = pf as isize;
            while point as usize >= 1 {
                intensity[(point as usize)+shift_abs]=intensity[(point as usize)];
                intensity[(point as usize)]=0.0;

                point-=1;  // Decrement
            }  // for(i=pf;i>=1;i--)
        }  // if shift...

        // ...
        // Stickspectrum is now stored in intensity vector;
        // It's time for the Fourier transformation of the Stickspectrum...
        // ... and multiplication with the Fourier transform of the lineshape function.

        let mut t2 = 2.0/(3.0_f64.sqrt())*rad.lwa.val;  // Lorentzian lineshape

        let mut t1 = (-0.02)*(t2.powi(3))*rad.amount.val*rad.lrtz.val /
            (totale as f64*std::f64::consts::PI);  // Gaussian lineshape

        let mut w2 = -(sweep as f64)/2.0;

        let mut point = 1;
        while point < points as usize {
            let a = w2-rad.dh1.val;
            // Peak intensity!
            lno[point] = (t1*a)/((1.0+t2.powi(2)*a.powi(2))*(1.0+t2.powi(2)*a.powi(2)));
            w2 = w2 + incrgauss as f64;

            point+=1;  // Increment point
        }  // for (j=1;j<=punti;j++)

        w2 = -(sweep as f64)/2.0; // reset w2
        t2 = 2.0/rad.lwa.val;  // change t2

        t1 = -rad.amount.val*(t2.powi(3))*0.01*(100.0-rad.lrtz.val)/
            (totale as f64*(2.0*std::f64::consts::PI).sqrt());  // 100-lorentz == gauss

        let mut point = 1;
        while point < points as usize {
            let a = w2-rad.dh1.val;
            let dd = (std::f64::consts::E).powf(-0.5*(t2.powi(2))*(a.powi(2)));
            if dd > 1E-35 { lno[point] = lno[point] + t1*a*dd; }
            w2 = w2 + incrgauss as f64;

            point+=1;  // Increment point
        }  // for (j=1;j<=punti;j++)

        point = 1;  // Restart counter
        while point < points as usize {
            if intensity[point] != 0.0 {
                let mut i1 = 1;
                while i1 < points as usize {
                    let i2: isize = (points as isize/2) - i1 as isize;
                    if ((point as isize -i2) >= 1) && ((point as isize -i2) < (points as isize)) {
                        newteor[(point as isize -i2 as isize) as usize]+=(lno[i1] as f64)*(intensity[point] as f64);
                    }

                    i1+=1;  // Increment 1i
                }  // for (i1=1;i1<=punti;i1++)
            }  // if intensity[point]

            point+=1; // Increment j
        }  // for (j=1;j<=punti;j++)
    }

    // println!("{:?}", newteor);
    newteor  // return
}  // fn calcola