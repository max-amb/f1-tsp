pub fn calculate(long1: f64, lat1: f64, long2: f64, lat2: f64) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    return earth_radius_kilometer*((lat1.cos()*lat2.cos()*((long1-long2).cos())+(lat1.sin()*lat2.sin())).acos());
}
