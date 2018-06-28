extern crate nesutil;
extern crate raster;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let image = raster::open("test.png").unwrap();

    let pt = nesutil::PatternTable::load_from_image(&image).unwrap();

    let mut file = File::create("test.chr").unwrap();
    file.write_all(&pt.data).ok();
}
