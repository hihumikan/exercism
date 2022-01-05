// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1 => 221.0,
        2 => 221.0*2.0,
        3 => 221.0*3.0,
        4 => 221.0*4.0,
        5 => 221.0*5.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    unimplemented!("calculate the amount of working items at speed: {}", speed)
}
