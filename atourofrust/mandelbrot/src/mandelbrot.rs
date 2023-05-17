use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use num::Complex;
use std::str::FromStr;

pub fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>{

    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

pub fn render(pixels: &mut [u8], 
        bounds: (usize, usize), 
        upper_left: Complex<f64>,
        lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255-count as u8
                };
        }
    }
}

pub fn pixel_to_point(bounds: (usize, usize),
                pixel: (usize, usize),
                upper_left: Complex<f64>,
                lower_right: Complex<f64>) -> Complex<f64> {

    let (width, height) = (lower_right.re - upper_left.re, 
        upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }

}



pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => {
            match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}



pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
         None => None,
         Some((real, imag)) => Some(Complex::new(real, imag))
    }
}

pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);   
        }
        z = z * z + c;
    }
    None
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);    
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}


#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,10.32"), Some(Complex::new(1.25, 10.32)));
    assert_eq!(parse_complex(",-0.07686787"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                                Complex{re: -1.0, im: 1.0},
                                Complex{re: 1.0, im: -1.0}),
                                Complex{re: -0.5, im: -0.75});
}