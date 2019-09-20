use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;

#[derive(Clone)]
pub struct MapData {
    pub rows: u32,
    pub cols: u32,
    pub min_elevation: f32,
    pub max_elevation: f32,
    pub values: Vec<f32>,
}

impl MapData {
    pub fn open(path: &str, rows: u32, cols: u32) -> std::io::Result<MapData> {
        let mut map_data = MapData {
            rows,
            cols,
            min_elevation: std::f32::MAX,
            max_elevation: std::f32::MIN,
            values: vec![],
        };

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut point_buffer = [0; 4];

        loop {
            match reader.read(&mut point_buffer)? {
                // no more data to read; we're full!
                0 => return Ok(map_data),

                // still reading data ... convert the buffer into a point
                _ => {
                    let point = Self::buffer_to_float(point_buffer);

                    // adjust min/max elevation values
                    if point < map_data.min_elevation {
                        map_data.min_elevation = point;
                    }
                    if point > map_data.max_elevation {
                        map_data.max_elevation = point;
                    }

                    // save the point
                    map_data.values.push(point);
                }
            }
        }
    }

    pub fn elevation_slice(&self, elevation: f32) -> MapData {
        let mut slice = self.clone();

        slice.min_elevation = 0.0;
        slice.max_elevation = std::f32::MAX;

        for point in slice.values.iter_mut() {
            if *point < elevation {
                *point = slice.min_elevation;
            }

            if *point >= elevation {
                *point = slice.max_elevation;
            }
        }

        slice
    }

    pub fn is_empty(&self) -> bool {
        for point in self.values.iter() {
            if *point > self.min_elevation {
                return false;
            }
        }

        true
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&f32> {
        self.values.get(self.index_of(x, y))
    }

    fn index_of(&self, x: u32, y: u32) -> usize {
        (x + (y * self.rows)).try_into().unwrap()
    }

    fn buffer_to_float(buffer: [u8; 4]) -> f32 {
        unsafe { mem::transmute::<[u8; 4], f32>(buffer) }
    }
}
