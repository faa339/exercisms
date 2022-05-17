// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // figure out the speed, put results in an f64 and return
    let basespeed = (speed)as f64 * 221.0;
    if speed > 0 && speed <= 4{
        return basespeed as f64; 
    }else if speed>=5 && speed<=8{
        return ((basespeed)as f64 * 0.9) as f64;
    }else{
        return ((basespeed)as f64 * 0.77) as f64;
    } 
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    //use the speed to get the pph then / 60? 
    return (production_rate_per_hour(speed) / 60.0) as u32;
}
