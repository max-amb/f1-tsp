/// This file contains the calculate function to work out distances

/// This function calculates the distance between two sets of coordinates on a globe
/// Args:
///     long1: f64, lat1: f64 - the first set of coordinates
///     long2: f64, lat2: f64 - the second set of coordinates
pub fn calculate(long1: f64, lat1: f64, long2: f64, lat2: f64) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    earth_radius_kilometer*((lat1.cos()*lat2.cos()*((long1-long2).cos())+(lat1.sin()*lat2.sin())).acos())
}
