// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production_rate;
    if speed < 5 {
        production_rate = speed as f64 * 221.0;
    }
    else if speed < 9{
        production_rate = speed as f64 * 221.0 * 0.9;
    }
    else {
        production_rate = speed as f64 * 221.0 * 0.77;
    }
    return production_rate;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    0
}
