// Test compression algorithms for numbers / data
// Written 2016.04.05 by Willi Kappler (grandor@gmx.de)

// External crates:
// none

// Internal crates:
extern crate compression1;

// System modules:
// none

// External modules:
// none

// Internal modules:
use compression1::*;


fn main() {
    let y1 = compress_year(2016);
    let d1 = compress_date(2016, 4, 5); // 2016.04.05
    let t1 = compress_time(15, 21); // 10:21 h
    let dt1 = compress_date_time(2016, 4, 5, 15, 21); // 2016.04.05, 15:21

    println!("Year: {:0>8x}, {:0>32b}", y1, y1);
    println!("Date: {:0>8x}, {:0>32b}", d1, d1);
    println!("Time: {:0>8x}, {:0>32b}", t1, t1);
    println!("DT  : {:0>8x}, {:0>32b}", dt1, dt1);

    println!("Decoded: {:?}", decode_date_time(dt1));
}
