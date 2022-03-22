use crate::Context;

pub fn draw_classic(cr: &Context, peaks: &Vec<f32>, w: f64, h: f64) {
    // Study how this scaling function works exactly (probably not needed)
    // cr.scale(500f64, 500f64);

    // Yellow background
    cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
    cr.paint().expect("Invalid cairo surface state");

    // Line drawer settings
    cr.set_line_width(1.0);
    cr.set_source_rgb(0.3, 0.3, 0.3);

    let w = w as f32;
    let h = h as f32;
    let verti_center = h/2.0;
    let horiz_center = w/2.0;

    // let theor_min = (peaks.iter().fold(f64::INFINITY, |a, &b| a.min(b.into()))) as f32;
    // let theor_max = (peaks.iter().fold(f64::INFINITY, |a, &b| a.max(b.into()))) as f32;
    // let verti_center = h as f32 / (theor_max-theor_min);
    // let theor_min = 0.0;
    // let theor_max = 100.0;

    // CLASSIC 1999 PLOTTING ITERATION STRATEGY
    let x_incr = (w as f32) / 1024.0;
    // let y_incr = (h as f32) / (theor_max-theor_min);
    // println!("y_incr is: {}", y_incr);

    let mut x1 = 0.0 as f32;

    cr.move_to(x1.into(), 0.0);

    for point in 0..1024 {
        // pointer
        let p_from = verti_center as f32 + peaks[point]; // * y_incr;
        let p_to = verti_center as f32 - peaks[point]; // * y_incr;
        let x2 = x1 as f32 + (1.0 as f32 *x_incr);
        // println!("p_from {}, p_to {}", p_from, p_to);
        cr.move_to(x1.into(), p_from.into());
        cr.line_to(x2.into(), p_to.into());
        x1=x2;
    }

    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.stroke().expect("invalid cairo surface state");
}
