/// This file does the parsing of the f1-locations.json
use std::fs::File;
use std::env::current_exe;
use std::io::BufReader;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

/// The location struct which contains all of the information relavent for a location
#[derive(Serialize, Deserialize,Clone)]
pub struct Location {
    pub lon: f64,
    pub lat: f64,
    pub location: String,
    pub id: String,
}

/// This function opens the file, iterates through its lines and adds it to the locations json
pub fn parse_json() -> Result<Vec<Location>> {
    let mut locations: Vec<Location> = Vec::new();

    // This is so awkward due to the exe being in a target directory far away from the json
    let mut path_exe = current_exe().unwrap();
    path_exe.pop();
    path_exe.pop();
    path_exe.pop();
    path_exe.push("data/f1-locations.json");

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(path_exe.clone()) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(_) => panic!("couldn't open file: {:?}", path_exe.to_str()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for lines in reader.lines() {
        let tmp: Location = serde_json::from_str(lines.unwrap().as_str())?;
        locations.push(tmp);
    }
    Ok(locations)
}
