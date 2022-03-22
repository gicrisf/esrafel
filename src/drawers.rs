use crate::Context;



pub fn paint_bg(cr: &Context) {
    // Yellow background
    cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
    cr.paint().expect("Invalid cairo surface state");
}

// This works with spectra and doesn't with the noise
pub fn draw_classic(cr: &Context, peaks: &Vec<f64>, w: f64, h: f64) {
    // Study how this scaling function works exactly (probably not needed)
    // cr.scale(500f64, 500f64);

    // Line drawer settings
    cr.set_line_width(1.0);
    // TODO remove
    cr.set_source_rgb(0.3, 0.3, 0.3);

    // add to model
    // let verti_center = h/2.0;
    // let horiz_center = w/2.0;

    let theor_min = peaks.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    // b theor_max = (peaks.iter().fold(f64::INFINITY, |a, &b| a.max(b)));
    // let theor_max = peaks.iter().max_by(|a, b| a.total_cmp(b)); // unstable!
    let theor_max = peaks
        .iter()
        .copied()
        .map(ordered_float::NotNan::new)
        .flatten() // ignore NAN values (errors from the previous line)
        .max()
        .map(ordered_float::NotNan::into_inner)
        .expect("cannot find max of vector");

    // println!("max: {:?}, min: {:?}", theor_max, theor_min);
    // let verti_center = h as f32 / (theor_max-theor_min);
    // let theor_min = 0.0;
    // let theor_max = 100.0;

    // CLASSIC 1999 PLOTTING ITERATION STRATEGY
    let x_incr = (w) / 1024.0;
    let y_incr = (h) / (theor_max-theor_min);
    // println!("y_incr is: {}", y_incr);

    let mut x1 = 0.0;
    let mut p_from = (theor_max - peaks[0]) *y_incr;
    // println!("{:?}", p_from);

    cr.move_to(x1, p_from);

    for point in 0..1023 {
        // pointer
        // let p_from = verti_center as f32 + peaks[point]; // * y_incr;
        // let p_to = verti_center as f32 - peaks[point]; // * y_incr;
        let p_to = (theor_max - peaks[point+1])*y_incr;
        // let x2 = x1 + (1.0 *x_incr);
        let x2 = x1 + x_incr;
        // println!("p_from {}, p_to {}", p_from, p_to);
        cr.move_to(x1, p_from);
        cr.line_to(x2, p_to);
        x1=x2;
        p_from=p_to;
    }

    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.stroke().expect("invalid cairo surface state");
}


pub fn draw_noise(cr: &Context, peaks: &Vec<f64>, w: f64, h: f64) {
    // Line drawer settings
    cr.set_line_width(1.0);
    cr.set_source_rgb(0.3, 0.3, 0.3);

    let verti_center = h/2.0;
    let horiz_center = w/2.0;

    let x_incr = w / 1024.0;
    // let y_incr = (h as f32) / (theor_max-theor_min);
    // println!("y_incr is: {}", y_incr);

    let mut x1 = 0.0;

    cr.move_to(x1, 0.0);

    for point in 0..1024 {
        // pointer
        let p_from = verti_center + peaks[point]; // * y_incr;
        let p_to = verti_center - peaks[point]; // * y_incr;
        let x2 = x1 + (1.0 *x_incr);
        // println!("p_from {}, p_to {}", p_from, p_to);
        cr.move_to(x1, p_from);
        cr.line_to(x2, p_to);
        x1=x2;
    }

    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.stroke().expect("invalid cairo surface state");
}
