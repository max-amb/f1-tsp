use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize,Clone)]
pub struct Location {
    pub lon: f64,
    pub lat: f64,
    pub location: String,
    pub id: String,
}

pub fn parse_json() -> Result<Vec<Location>> {
    let mut locations: Vec<Location> = Vec::new();
    let path = Path::new("/home/max/Documents/code/f1-calendar/src/f1-locations.json");
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(_) => panic!("couldn't open file"),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for lines in reader.lines() {
        let tmp: Location = serde_json::from_str(lines.unwrap().as_str())?;
        locations.push(tmp);
    }
    Ok(locations)
}
