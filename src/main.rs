// #![allow(unused_imports, unused_variables)]
use image::{self, GenericImageView, ImageBuffer, Rgb};
use std::cmp::Ordering;

fn main() {
    const THRESHHOLD: f32 = 20.0;
    let img_path = "image.jpg";
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let buffer = img.to_rgb8();

    let mut out_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let mut out_vec: Vec<Rgb<u8>> = Vec::new();

    let rows = buffer.rows();
    for row in rows {
        let row_clone: Vec<Rgb<u8>> = row.clone().copied().collect();
        let rowed: Vec<Rgb<u8>> = row.copied().collect();
        let mut current_index: usize = 0;
        let mut current_sort_buff: Vec<Rgb<u8>> = vec![];
        for pixel in rowed {
            if luma_from_pixel(&pixel) > THRESHHOLD {
                current_sort_buff.push(pixel);
            }
            else {
                if current_sort_buff.len() > 0{
                    current_sort_buff.sort_by(|a, b| comp_pixel(a, b));
                    // println!("appending {:?} pixels", current_sort_buff.len());
                    out_vec.append(&mut current_sort_buff);
                    
                }
                out_vec.push(row_clone[current_index]);
                // println!("Added single pixel");
            }
            current_index += 1;
        }
        current_sort_buff.sort_by(|a, b| comp_pixel(a, b));
        // println!("appending {:?} pixels", current_sort_buff.len());
        out_vec.append(&mut current_sort_buff);
        // println!("Row size: {:?}", row_clone.len());
        // println!("Buffer size: {:?}", out_vec.len());

        // rowed.sort_by(|a, b| comp_pixel(a, b));
        // out_vec.append(&mut rowed);
    }

    for (i, pixel) in out_buffer.pixels_mut().enumerate() {
        *pixel = out_vec[i];
    }

    out_buffer.save("output.png").expect("Failed to save image");
}

fn comp_pixel(a: &Rgb<u8>, b: &Rgb<u8>) -> Ordering {
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

fn luma_from_pixel(pixel: &Rgb<u8>) -> f32 {
    let r: f32 = pixel[0].into();
    let g: f32 = pixel[1].into();
    let b: f32 = pixel[2].into();

    let x: f32 = r + g + b;
    x / 3.0
}
