//use image::RgbImage;
use image::{DynamicImage, ImageBuffer};
use std::env;
use std::fs;

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn buffer_to_img(
    data: &[u8],
    colour_map: &[image::Rgb<u8>; 2],
    byte_ptr: usize,
    // bit_offset: u8,
    height: usize,
) -> Option<DynamicImage> {
    let mut raw_rgb_buf: Vec<image::Rgb<u8>> = Vec::with_capacity(height * 4);

    for byte in &data[byte_ptr..(byte_ptr + (height >> 1))] {
        for bit_pos in (0..8).rev() {
            if byte & (1 << bit_pos) == 0 {
                raw_rgb_buf.push(colour_map[0]);
            } else {
                raw_rgb_buf.push(colour_map[1])
            }
        }
    }

    let flat_buf: Vec<u8> = raw_rgb_buf.iter().flat_map(|rgb| rgb.0.to_vec()).collect();

    let width = 4;
    if flat_buf.len() == width * height * 3 {
        ImageBuffer::from_raw(width as u32, height as u32, flat_buf).map(DynamicImage::ImageRgb8)
    } else {
        None
    }
}

fn main() {
    //read random_bytes.bits and cmd arguments
    let args: Vec<String> = env::args().collect();

    let data: Vec<_> = fs::read("pi.bits").unwrap();
    let mapped_data: Vec<u8> = vec![0b11111001, 0b11111001];
    let colour_map = [image::Rgb([2, 52, 63]), image::Rgb([240, 237, 204])];
    let mut height: usize = 8;
    if args.len() > 1 {
        height = args[1].parse().unwrap();
    }
    let sus_position = find_subsequence(&data, &mapped_data).unwrap();
    println!("{}", sus_position);
    let pi_img = buffer_to_img(&data, &colour_map, sus_position - 1, height).unwrap();
    pi_img.save("pi_sec.png").unwrap();
}
