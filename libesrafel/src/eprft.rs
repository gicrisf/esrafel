use crate::{Radical};

// Calculate theoretical spectra
pub fn calcola(rads: &Vec<Radical>, sweep: f64, points: f64) -> Vec<f64> {
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
            pa += pcostanti[i] * spini[i] * nuc.eqs.val;
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

                    indice1 -= 1; // Decrement
                }  // while indice1...

                eq+=1;
            }  // for(eq=1;eq<=nucleis[l][i];i1++)
        }  // for nuc in rad.nucs

        let shift: isize = ((points as isize)-(pf as isize))/2;
        let shift_abs: usize = shift.unsigned_abs();  // Eraseme

        // TODO refactor with match
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
            w2 += incrgauss as f64;

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
            if dd > 1E-35 { lno[point] += t1*a*dd; }
            w2 += incrgauss as f64;

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

    newteor  // return
}  // fn calcola

// MONTECARLO

// **Strict** porting of classic Montecarlo functions of ESR Commander 1999
pub fn errore(
    exp: &[f64],
    points: f64,
    mut newteor: Vec<f64>) -> (f64, Vec<f64>) {

    let (mut somma, mut somma1, mut somma2): (f64, f64, f64) = (0.0, 0.0, 0.0);
    let start: usize = 1;
    let fine = points as usize;  // TODO check this one

    // Start MC
    for j in start..fine {
        somma1 += newteor[j].powi(2);
        somma2 += exp[j].abs() * newteor[j].abs();
    }

    let norma: f64;
    if somma1 != 0.0 {
        norma = somma2/somma1
    } else {
        norma = 0.0
    };

    for j in start..fine {
        newteor[j] *= norma;
        let diff = (exp[j] - newteor[j]).powi(2);
        somma+=diff;
    }

    let newsigma =(somma/(fine-start) as f64).sqrt();

    (newsigma, newteor)
}  // mc

// TODO use better var names
// Temporarily maintaining legacy name to make easier comparison

// Reset potentially aberrant value returned by MC function;
fn check_pars(mut rad: Radical) -> Radical {
    if rad.lwa.val < 0.0 { rad.lwa.val = 0.0 };
    if rad.lrtz.val < 0.0 { rad.lrtz.val = 0.0 };
    if rad.amount.val < 0.0 { rad.amount.val = 0.0 };
    if rad.lrtz.val > 100.0 { rad.lrtz.val = 100.0 };
    rad
}

fn caso(rads: &[Radical]) -> Vec<Radical> {
    let mut mc_rads = Vec::new();

    for mut rad in rads.iter().cloned() {
        rad.lwa = rad.lwa.randomize();
        rad.amount = rad.amount.randomize();
        rad.lrtz = rad.lrtz.randomize();
        rad.dh1 = rad.dh1.randomize();

        let mut randomized_nucs = Vec::new();
        for mut nuc in rad.nucs.iter().cloned() {
            nuc.hpf = nuc.hpf.randomize();
            randomized_nucs.push(nuc);
        }

        rad.nucs = randomized_nucs;

        // TODO check this out
        rad = check_pars(rad);
        mc_rads.push(rad);
    }  // for rad in rads
    mc_rads
}

// This is my way to wrap MC logic in a single function
// This return a simple tuple, so we maintain pure functional paradigm
pub fn mc_fit(
    empirical: &[f64],
    points: f64,
    sweep: f64,
    mut sigma: f64,
    mut rads: Vec<Radical>) -> (f64, Vec<f64>, Vec<Radical>) {

    // Randomize parameters for next iteration
    let newrads = caso(&rads);

    // Reallocate params if variance is less than previous iteration
    let (newsigma, newteor) = errore(
        empirical, points,
        calcola(&newrads, sweep, points)
    );

    if newsigma < sigma {
        sigma = newsigma;
        rads = newrads;
    }

    // Return newteor nevertheless: you plot it anywhere
    (sigma, newteor, rads)
}
