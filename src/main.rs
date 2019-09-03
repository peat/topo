use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::io::{BufWriter, BufReader};

const ROWS: usize = 10812; // number of samples
const COLS: usize = 10812;
const MAX_ELEVATION: f32 = 3500.0; // in meters
const SLICE_HEIGHT: f32 = 10.0; // in meters

fn main() -> std::io::Result<()> {
    let mut input_data = BufReader::new(File::open("data/usgs_ned_13_n46w122_gridfloat.flt")?);
    let output_file = File::create("example.png")?;

    let mut point_buffer = [0; 4];
    let mut image_buffer: Vec<u8> = vec![];
    image_buffer.reserve(COLS * ROWS * 4);

    while let Ok(bytes) = input_data.read(&mut point_buffer) {
        if bytes == 0 {
            break;
        }

        let float_value = to_float(point_buffer);
        let pct_max = float_value / MAX_ELEVATION;
        let rgb: u8 = (pct_max * 255.0) as u8;
        let pixel = match rgb % 10 {
            0 => [255, 0, 0, 255],
            _ => [rgb, rgb, rgb, 255],
        };

        image_buffer.extend_from_slice(&pixel);
    }

    let mut encoder = png::Encoder::new(BufWriter::new(output_file), COLS as u32, ROWS as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut image_writer = encoder.write_header().unwrap();

    image_writer.write_image_data(&image_buffer)?;

    println!("{:?}", image_buffer.len());

    Ok(())
}

fn to_float(i: [u8; 4]) -> f32 {
    unsafe { mem::transmute::<[u8; 4], f32>(i) }
}
