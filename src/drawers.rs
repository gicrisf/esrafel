use crate::Context;
use ordered_float::NotNan;

// TODO Use gdk4::RGBA instead!
// https://gtk-rs.org/gtk4-rs/stable/latest/docs/gdk4/struct.RGBA.html
#[derive(Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color {
            r: r / 255.0,
            g: g / 255.0,
            b: b / 255.0,
        }
    }

    pub fn _original(name: &str) -> Color {
        match name {
            "DarkCyan" => Color::rgb(1.0, 46.0, 64.0),
            "LightCyan" => Color::rgb(79.0, 134.0, 140.0),
            _ => Color::rgb(0.0, 0.0, 0.0),  // Return black if unknown
        }
    }

    pub fn _solarized(name: &str) -> Color {
        match name {
            "White" => Color::rgb(238.0, 232.0, 213.0),
            "Orange" => Color::rgb(203.0, 75.0, 22.0),
            "Cyan" => Color::rgb(79.0, 134.0, 140.0),
            "Violet" => Color::rgb(108.0, 113.0, 196.0),
            "Yellow" => Color::rgb(181.0, 137.0, 0.0),
            _ => Color::rgb(0.0, 0.0, 0.0),  // Return black if unknown
        }
    }

    fn as_tuple(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }
}

#[derive(Default)]
pub struct Line {
    pub data: Vec<f64>,
    pub length: usize,
    pub max: f64,
    pub min: f64,
}

impl Line {
    pub fn new(data: Vec<f64>) -> Self {
        let length = data.len();
        let max = data
            .iter()
            .copied()
            .map(NotNan::new)
            .flatten() // ignore NAN values (errors from the previous line)
            .max()
            .map(NotNan::into_inner)
            .expect("Cannot define max of Line");

        let min = data
            .iter()
            .copied()
            .map(NotNan::new)
            .flatten() // ignore NAN values (errors from the previous line)
            .min()
            .map(NotNan::into_inner)
            .expect("Cannot define min of Line");


        Self {
            data,
            length,
            max,
            min
        }
    }
}  // Line struct

pub fn paint_bg(cr: &Context, color: &Color) {
    let (a, b, c) = color.as_tuple();
    cr.set_source_rgb(a, b, c);
    // cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
    cr.paint().expect("Invalid cairo surface state");
}

// CLASSIC DOS 1999 PLOTTING ITERATION STRATEGY
//
// SIM99.C By Marco Lucarini and Gian Franco Pedulli
// Dipartimento di Chimica Organica, Facoltà di Farmacia Università di Bologna.
//
// ```clang
// spettro(float p[],float pm[],int init, int fine, ...)
// {
// float yincr,xincr,x1,x2,max,min;
// int i,p1,p2;
// ...
// // Find min/max through a simple iteration
// ...
// xincr=630.0/((float) (fine-init));
// yincr=440.0/(max-min);
// setviewport(5,20,635,460,1);
// ...
// x1=0.0;
// p1=(int) ((max-p[init]*molt)*yincr);
// for (i=init+1;i<=fine;i++)
// {
// p2=(int) ((max-p[i]*molt)*yincr);
// x2=x1+xincr;
// line(x1,p1,x2,p2);
// x1=x2;
// p1=p2;
// }  // ./for
// }  // ./spettro
// ```

pub fn draw_classic(cr: &Context, line: &Line, w: f64, h: f64, color: &Color) {
    cr.set_line_width(1.0);
    let (a, b, c) = color.as_tuple();
    cr.set_source_rgb(a, b, c);

    let x_incr = w / (line.length as f64);
    let y_incr = h / (line.max-line.min);

    // Move to initial x, y coords
    let mut x1 = 0.0;
    let mut p_from = (line.max - line.data[0]) *y_incr;
    cr.move_to(x1, p_from);

    // Scale and plot line
    for point in 0..(line.length-1) {
        let p_to = (line.max - line.data[point+1])*y_incr;
        let x2 = x1 + x_incr;
        cr.move_to(x1, p_from);
        cr.line_to(x2, p_to);
        x1=x2;
        p_from=p_to;
    }

    cr.stroke().expect("invalid cairo surface state");
}


pub fn _draw_noise(cr: &Context, line: &Line, w: f64, h: f64, color: &Color) {
    cr.set_line_width(1.0);
    let (a, b, c) = color.as_tuple();
    cr.set_source_rgb(a, b, c);

    let verti_center = h/2.0;
    // let horiz_center = w/2.0;
    let x_incr = w / (line.length as f64);

    let mut pointer = 0.0;
    cr.move_to(pointer, 0.0);

    for point in 0..(line.length) {
        let p_from = verti_center + line.data[point];
        let p_to = verti_center - line.data[point];
        let x2 = pointer + (1.0 *x_incr);
        cr.move_to(pointer, p_from);
        cr.line_to(x2, p_to);
        pointer=x2;
    }

    cr.stroke().expect("invalid cairo surface state");
}
