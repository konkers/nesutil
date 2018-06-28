extern crate nesutil;
extern crate raster;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn load_from_image_test() {
    let image = raster::open("tests/data/tiles.png").unwrap();

    let pt = nesutil::PatternTable::load_from_image(&image).unwrap();

    let mut golden_output = [0; 4096];
    let mut file = File::open("tests/data/tiles.chr").unwrap();
    file.read(&mut golden_output).ok();

    for i in 0..4096 {
        assert_eq!(
            golden_output[i], pt.data[i],
            "Output mismatch at {}: expected {}, got {}",
            i, golden_output[i], pt.data[i],
        );
    }
}
