extern crate raster;
#[macro_use]
extern crate simple_error;

use raster::filter;
use simple_error::SimpleError;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::error::Error;

const TILES_WIDE: i32 = 16;
const TILES_TALL: i32 = 16;

const TILE_HEIGHT: i32 = 8;
const TILE_WIDTH: i32 = 8;

const TILES_PER_PAGE: i32 = 256;
const BYTES_PER_TILE: i32 = 16;
const PAGE_SIZE: usize = (TILES_PER_PAGE * BYTES_PER_TILE) as usize;

fn tile_coord(tile: i32) -> (i32, i32) {
    let x = tile % TILES_WIDE;
    let y = (tile - x) / TILES_TALL;

    (x * TILE_WIDTH, y * TILE_HEIGHT)
}

pub struct PatternTable {
    pub data: [u8; PAGE_SIZE],
}

impl PatternTable {
    pub fn load_from_image(source_image: &raster::Image) -> Result<PatternTable,  Box<Error>> {
        let mut image = source_image.clone();

        if image.width != 128 || image.height != 128 {
            return Err(From::from(format!(
                "Image is {}x{}.  Needs to be 128x128",
                image.width,
                image.height
            )));
        }

        if filter::grayscale(&mut image).is_err() {
            return Err(From::from("Can't convert image to greyscale."));
        }

        let mut colors = BTreeSet::new();

        for x in 0..image.width {
            for y in 0..image.height {
                colors.insert(image.get_pixel(x, y).unwrap().r);
            }
        }

        if colors.len() > 4 {
            return Err(From::from(format!("image has {} colors.  Needs to be 4 or less.", colors.len())));
        }

        let mut color_map = HashMap::new();
        let mut i = 0;
        for val in colors {
            color_map.insert(val, i);
            i = i + 1;
        }

        let mut pt = PatternTable {
            data: [0; PAGE_SIZE],
        };

        for tile in 0..TILES_PER_PAGE {
            let (tile_x, tile_y) = tile_coord(tile);
            for y in 0..TILE_HEIGHT {
                let mut high: u8 = 0;
                let mut low: u8 = 0;
                for x in 0..TILE_WIDTH {
                    let image_val = image.get_pixel(tile_x + x, tile_y + y).unwrap().r;
                    let val = color_map.get(&image_val).unwrap();
                    let chr_bit = (1 << (TILE_WIDTH - x - 1)) as u8;
                    if val & 0x1 != 0 {
                        low |= chr_bit;
                    }
                    if val & 0x2 != 0 {
                        high |= chr_bit;
                    }
                }
                let offset = tile * BYTES_PER_TILE + y;
                pt.data[offset as usize] = low;
                pt.data[(offset + BYTES_PER_TILE / 2) as usize] = high;
            }
        }
        Ok(pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tile_coord() {
        assert_eq!((0, 0), tile_coord(0));
        assert_eq!((7 * 8, 0), tile_coord(7));
        assert_eq!((15 * 8, 0), tile_coord(15));
        assert_eq!((0, 1 * 8), tile_coord(16));
        assert_eq!((15 * 8, 15 * 8), tile_coord(255));
    }
}