// DATA AT https://www.usgs.gov/core-science-systems/ngp/tnm-delivery

mod depth_image;
mod map_data;

use depth_image::*;
use map_data::*;

const ROWS: u32 = 10812; // number of samples
const COLS: u32 = 10812;
const STEP_SIZE: f32 = 50.0; // meters

struct Elevator {
    current: f32,
    maximum: f32,
    step: f32,
}

impl Elevator {
    pub fn new(start: f32, maximum: f32, step: f32) -> Elevator {
        Elevator {
            maximum,
            current: start,
            step,
        }
    }
}

impl Iterator for Elevator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.current += self.step;
        if self.current > self.maximum {
            None
        } else {
            Some(self.current)
        }
    }
}

fn main() -> std::io::Result<()> {
    // let map_data = MapData::open("data/usgs_ned_13_n46w122_gridfloat.flt", ROWS, COLS)?; // hood
    let map_data = MapData::open("data/usgs_ned_13_n47w123_gridfloat.flt", ROWS, COLS)?; // helens

    let elevator = Elevator::new(map_data.min_elevation, map_data.max_elevation, STEP_SIZE);

    for (index, elevation) in elevator.enumerate() {
        let map_slice = map_data.elevation_slice(elevation);
        if map_slice.is_empty() {
            continue;
        }

        let image_data = DepthImage::from(&map_slice);
        let image_path = format!("output/example-{:04}.png", index);

        image_data.write(&image_path)?;

        println!("{:?}", image_path);
    }

    Ok(())
}
