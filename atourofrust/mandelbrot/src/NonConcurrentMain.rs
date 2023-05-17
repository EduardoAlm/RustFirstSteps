use std::env;
mod mandelbrot;
use mandelbrot::parse_pair;
use mandelbrot::parse_complex;
use mandelbrot::render;
use mandelbrot::write_image;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]); 
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]); 
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Could not parse bounds");
    let upper_left = parse_complex(&args[3]).expect("Could not parse upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Could not parse lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("Could not write image");
}

