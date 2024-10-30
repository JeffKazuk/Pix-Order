// #![allow(unused_imports, unused_variables)]
use clap::Parser;
use image::{self, GenericImageView, ImageBuffer, Rgba};
use std::cmp::Ordering;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "20.0")]
    threshold: f32,
    #[arg(short, long, default_value = "image.jpg")]
    input: String,
    #[arg(short, long, default_value = "output.png")]
    output: String,
    #[arg(short, long, default_value = "right")]
    direction: String,
}

// This is needed otherwise trying to use the struct in nested
//      loops results in a move error
#[derive(Clone, Copy)]
// Simple reversible range
struct DimRange {
    max: u32,
    current: u32,
    descend: bool,
}
// Allow the struct to be iterated over
impl Iterator for DimRange {
    // Define the type of the object we're returning from the iterator
    type Item = u32;
    // This is the general signature of the next function
    // This function has to be defined for the iterator trait
    fn next(&mut self) -> Option<Self::Item> {
        let current;

        if self.descend {
            current = self.current;
            if self.current > 0 {
                self.current -= 1;
            }
            return Some(current);
        } else {
            if self.current < self.max {
                current = self.current;
                self.current += 1;
                return Some(current);
            }
        }
        // If we reached the stop in whichever direction we return None
        None
    }
}

fn main() {
    let args = Args::parse();
    let img = image::open(args.input).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let _buffer = img.to_rgb8();

    let mut out_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let mut out_vec: Vec<Rgba<u8>> = Vec::<Rgba<u8>>::with_capacity((width*height) as usize);

    let y_iter: DimRange;
    let x_iter: DimRange;

    y_iter = DimRange {
        max: height,
        current: 0,
        descend: false,
    };
    x_iter = DimRange {
        max: width,
        current: 0,
        descend: false,
    };

    for y in y_iter {
        let mut current_buffer: Vec<image::Rgba<u8>> = vec![];
        for x in x_iter {
            let pixel = img.get_pixel(x, y);
            if luma_from_pixel(pixel) > args.threshold {
                current_buffer.push(pixel);
            } else {
                if current_buffer.len() > 0 {
                    current_buffer.sort_by(|&a, &b| comp_pixel(a, b));
                    out_vec.append(&mut current_buffer);
                }
                out_vec.push(pixel);
            }
        }
        current_buffer.sort_by(|&a, &b| comp_pixel(a, b));
        out_vec.append(&mut current_buffer);
    }

    for (i, pixel) in out_buffer.pixels_mut().enumerate() {
        *pixel = out_vec[i];
    }

    out_buffer.save(args.output).expect("Failed to save image");
}

fn comp_pixel(a: Rgba<u8>, b: Rgba<u8>) -> Ordering {
    let a_luma: f32 = luma_from_pixel(a);
    let b_luma: f32 = luma_from_pixel(b);
    if a_luma > b_luma {
        Ordering::Greater
    } else if a_luma < b_luma {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

// Simple function for evaluating each pixel for the purpose of sorting
// could probably be done better
fn luma_from_pixel(pixel: Rgba<u8>) -> f32 {
    let r: f32 = pixel[0].into();
    let g: f32 = pixel[1].into();
    let b: f32 = pixel[2].into();

    let x: f32 = r + g + b;
    x / 3.0
}
