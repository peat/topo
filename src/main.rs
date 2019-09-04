// DATA AT https://www.usgs.gov/core-science-systems/ngp/tnm-delivery

use rayon::prelude::*;

mod depth_image;
mod map_data;

use depth_image::*;
use map_data::*;

const ROWS: u32 = 10812; // number of samples
const COLS: u32 = 10812;
const STEP_SIZE: f32 = 50.0; // meters

struct Elevator {
    start: f32,
    maximum: f32,
    step: f32,
}

impl Elevator {
    pub fn new(start: f32, maximum: f32, step: f32) -> Elevator {
        Elevator {
            start,
            maximum,
            step,
        }
    }

    pub fn steps(&self) -> Vec<f32> {
        let mut output = vec![];
        let mut next_step = self.start;
        while next_step < self.maximum {
            output.push(next_step);
            next_step += self.step;
        }

        output
    }
}

fn main() -> std::io::Result<()> {
    // let flt_path = "data/usgs_ned_13_n47w123_gridfloat.flt"; // helens
    let flt_path = "data/usgs_ned_13_n46w122_gridfloat.flt"; // hood

    print!("Importing {} ... ", flt_path);
    let map_data = MapData::open(flt_path, ROWS, COLS)?;
    println!(
        "{} points. Elevation: {:.1}-{:.1} meters.",
        map_data.values.len(),
        map_data.min_elevation,
        map_data.max_elevation
    );

    println!("Extracting slices at {}m intervals ...", STEP_SIZE);
    let elevator = Elevator::new(map_data.min_elevation, map_data.max_elevation, STEP_SIZE);

    elevator
        .steps()
        .par_iter()
        .enumerate()
        .for_each(|(index, elevation)| {
            let map_slice = map_data.elevation_slice(*elevation);
            if map_slice.is_empty() {
                return;
            }

            let image_data = DepthImage::from(&map_slice);
            let image_path = format!("output/example-{:04}.png", index);

            if image_data.write(&image_path).is_err() {
                println!("Error saving {}!", image_path);
                return;
            };

            println!(" {:.1}m -> {}", elevation, image_path);
        });

    Ok(())
}
