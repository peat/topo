use crate::map_data::MapData;
use crate::edge_finder::EdgeFinder;

use std::fs::File;
use std::io::BufWriter;

pub struct DepthImage {
    pub width: u32,
    pub height: u32,
    pub values: Vec<u8>,
}

impl From<&MapData> for DepthImage {
    fn from(source: &MapData) -> DepthImage {
        let mut output = DepthImage {
            width: source.cols,
            height: source.rows,
            values: vec![],
        };

        for point in source.values.iter() {
            let pct_max = point / source.max_elevation;
            let rgb: u8 = (pct_max * 255.0) as u8;
            let pixel = [rgb, rgb, rgb, 255];

            output.values.extend_from_slice(&pixel);
        }

        output
    }
} 

impl From<&EdgeFinder> for DepthImage {
    fn from (source: &EdgeFinder) -> DepthImage {
        let mut output = DepthImage {
            width: source.width,
            height: source.width,
            values: vec![],
        };

        for point in source.values.iter() {
            let pixel = match point {
                true => [255, 255, 255, 255],
                false => [0, 0, 0, 255],
            };

            output.values.extend_from_slice(&pixel);
        }

        output        
    }
}

impl DepthImage {
    pub fn write(&self, path: &str) -> std::io::Result<()> {
        let output_file = File::create(path)?;

        let mut encoder = png::Encoder::new(BufWriter::new(output_file), self.width, self.height);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);

        let mut image_writer = encoder.write_header()?;
        image_writer.write_image_data(&self.values)?;

        Ok(())
    }
}
