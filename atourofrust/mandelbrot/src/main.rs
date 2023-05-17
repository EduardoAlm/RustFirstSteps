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
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands = Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("Could not write image");
}

