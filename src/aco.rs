// ANT COLONY OPTIMIZATION
use std::collections::HashMap;
use crate::{Edge, parse::Location};

pub fn ant_colony(graph: HashMap<String, Vec<Edge>>, locations: Vec<Location>, starting_location: String) -> Vec<String> {
    let mut path: Vec<String> = vec![starting_location];
    path
}
