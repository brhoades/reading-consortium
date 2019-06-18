extern crate num;
extern crate crossbeam;
extern crate num_cpus;
extern crate sdl2;

use std::str::FromStr;
use num::Complex;
use std::io::Write;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        writeln!(std::io::stderr(),
                 "Usage: {} UPPERLEFT LOWERRIGHT",
                 args[0]
        ).unwrap();
        writeln!(std::io::stderr(),
                 "EXAMPLE: {} -1.25,0.32, -1,0.20",
                 args[0]
        ).unwrap();
        std::process::exit(1);
    }

    let mut upper_left = parse_complex(&args[1])
        .expect("Error parsing upper left point.");
    let mut lower_right = parse_complex(&args[2])
        .expect("Error parsing lower right point.");

    // SDL dragons
    // https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/renderer-texture.rs#L24
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();
    let bounds = window.size();
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    /* I couldn't get this zoom logic feeling right easily.
                    upper_left = Complex {
                        re: (upper_left.re * (x as f64 / bounds.0 as f64)) / 2.0,
                        im: (upper_left.im * (y as f64 / bounds.1 as f64)) / 2.0
                    };
                    lower_right = Complex {
                        re: (lower_right.re * (x as f64 / bounds.0 as f64)) / 2.0,
                        im: (lower_right.im * (y as f64 / bounds.1 as f64) / 2.0)
                    };
                    */

                    let texture_creator = canvas.texture_creator();

                    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, bounds.0 as u32, bounds.1 as u32)
                        .map_err(|e| e.to_string()).unwrap();
                    texture.with_lock(None, |mut buffer: &mut [u8], pitch: usize| {
                        get_pixels(upper_left, lower_right, bounds, pitch, &mut buffer);
                    }).unwrap();
                    canvas.clear();

                    canvas.copy(&texture, None, Some(Rect::new(0, 0, bounds.0 as u32, bounds.1 as u32))).unwrap();
                    canvas.present();
                },
                _ => ()
            }
        }
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
    }

    None
}

/// Parse the string s as a coordinate pair. For example, "800x600", "x"
/// is (800, 600) with the separator "x".
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>(" ", ' '), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>(",", ','), None);
    assert_eq!(parse_pair::<i32>("10,10", 'x'), None);
    assert_eq!(parse_pair::<i32>("10.0,10", ','), None);
    assert_eq!(parse_pair::<i32>("10,10", ','), Some((10, 10)));
    assert_eq!(parse_pair::<i32>("10,5", ','), Some((10, 5)));
    assert_eq!(parse_pair::<f64>("10.10x3.23", 'x'), Some((10.10, 3.23)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given a row and column of a pixel, return the matching point
/// on the complex plane.
///
/// `bounds` is a pair giving width/height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in
///         the image.
/// `upper_left` and `upper_right` parameters are points on the complex
///   plane designating the area the image covers.
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
   -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // pixel.1 increases as we go down; imaginary component increases
        // as we go up
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im: 1.0 },
                              Complex { re: 1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.5 });
               
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>,
          pitch: usize) {
    assert!(pixels.len() == bounds.0 * bounds.1 * 3);

    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let point = pixel_to_point(bounds, (x, y),
                                       upper_left, lower_right);
            match escape_time(point, 255) {
                None => (),
                Some(c) => {
                    // https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/renderer-texture.rs#L24
                    let offset = y * pitch + x * 3;
                    pixels[offset] = 255 - c as u8;
                    pixels[offset + 1] = 255 - c as u8;
                    pixels[offset + 2] = 255 - c as u8;
                }
            }
        }
    }
}

fn get_pixels(upper_left: Complex<f64>, lower_right: Complex<f64>, u32_bounds: (u32, u32),
              pitch: usize, pixels: &mut [u8]) {
    let bounds = (u32_bounds.0 as usize, u32_bounds.1 as usize);
    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<&mut [u8]> =
        pixels.chunks_mut(rows_per_band * bounds.0 * 3).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / (bounds.0 * 3);
            let band_bounds = (bounds.0, height);
            let band_upper_left =
                pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height),
                               upper_left, lower_right);
            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right, pitch);
            });
        }
    }).unwrap();
}
