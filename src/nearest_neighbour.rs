/// This file contains the nearest neighbour algorithm to calculate the fastest way round the graph
use std::collections::HashMap;
use crate::{Edge, Location};

/// Taking in the graph, the locations and the starting location this function goes through all the
/// nodes find ing the fastest way round, assuming the next best node is the closest node to it
/// (not very correct)
pub fn nn(graph: &HashMap<String, Vec<Edge>>, locations: &[Location], starting_location: String) -> Vec<String> {
    let mut path: Vec<String> = vec![starting_location.to_string()];
    let mut current_location = &starting_location;
    for _ in 0..locations.len()-1{
        let mut min_distance = f64::INFINITY;
        let mut next_location = String::new();
        let possible_directions = graph.get_key_value(current_location).unwrap().1;
        for x in possible_directions {
            if x.weight < min_distance && !path.contains(&x.to) {
                next_location = x.to.to_string();
                min_distance = x.weight;
            }
        }
        path.push(next_location.to_string());
        current_location = &path[path.len()-1]
    }
    path
}
