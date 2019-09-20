use crate::map_data::*;

use std::convert::TryInto;

const THRESHOLD: f32 = 1.0;

pub struct EdgeFinder {
    pub width: u32,
    pub height: u32,
    pub values: Vec<bool>,
}

impl EdgeFinder {
    fn index_of(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)).try_into().unwrap()
    }
}

impl From<&MapData> for EdgeFinder {
    fn from(m: &MapData) -> EdgeFinder {
        let mut ef = EdgeFinder {
            width: m.cols,
            height: m.rows,
            values: vec![false; m.cols as usize * m.rows as usize],
        };

        // look for significant changes in X or Y orientation
        for x in 0..m.cols {
            for y in 0..m.rows {
                let center = m.get(x, y).unwrap();

                let north = m.get(x, y + 1).unwrap_or(center);
                let north_diff = (center - north).abs();

                let east = m.get(x + 1, y).unwrap_or(center);
                let east_diff = (center - east).abs();

                let south = m.get(x, y - 1).unwrap_or(center);
                let south_diff = (center - south).abs();

                let west = m.get(x - 1, y).unwrap_or(center);
                let west_diff = (center - west).abs();

                let edge = north_diff > THRESHOLD
                    || east_diff > THRESHOLD
                    || south_diff > THRESHOLD
                    || west_diff > THRESHOLD;

                let index = ef.index_of(x, y);
                ef.values[index] = edge;
            }
        }

        ef
    }
}
