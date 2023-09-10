// https://en.wikipedia.org/wiki/Nearest_neighbour_algorithm
use std::collections::HashMap;
use crate::{Edge, parse::Location};
pub fn nn(graph: HashMap<String, Vec<Edge>>, locations: Vec<Location>, starting_location: String) -> Vec<String> {
    let mut modgraph = graph.clone();
    let mut path: Vec<String> = vec![starting_location.clone()];
    let mut current_location = starting_location;
    for i in 0..locations.len()-1{
        let mut min_distance = f64::INFINITY;
        let mut next_location = String::new();
        let possible_directions = modgraph.get_key_value(current_location.as_str()).unwrap().1;
        for x in possible_directions {
            if x.weight < min_distance && !path.contains(&x.to) {
                next_location = x.to.to_string();
                min_distance = x.weight;
            }
        }
        path.push(next_location.clone());
        current_location = next_location.clone();
    }
    path
}
