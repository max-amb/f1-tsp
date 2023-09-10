// Not what I was looking for
use std::collections::HashMap;
use crate::{Edge, parse::Location};

// [x+locations.len()*y] method: https://users.rust-lang.org/t/attempt-to-use-a-non-constant-value-in-a-constant/32112/7

fn construct_matrix_for_floyds(locations: Vec<Location>) -> Vec<f64> {
    let mut matrix: Vec<f64> = vec![Default::default(); locations.len()*locations.len()];
    for x in 0..locations.len() {
        for y in 0..locations.len() {
            matrix[x+locations.len()*y] = f64::INFINITY;
        }
    }
    matrix
}

fn fill_matrix_for_floyds(locations: Vec<Location>, graph: HashMap<String, Vec<Edge>>) -> Vec<f64> {
    let mut minimum_distances = construct_matrix_for_floyds(locations.clone());
    for x in 0..locations.len() {
        for y in 0..locations.len() {
            let mut weight: f64 = f64::INFINITY;
            let edges = graph.get_key_value(locations[x].location.as_str()).unwrap().1;
            for edge in edges {
                if edge.to == locations[y].location {
                    weight = edge.weight;
                }
            }
            minimum_distances[x+locations.len()*y] = weight;
        }
    }
    for i in 0..locations.len() {
        minimum_distances[i+locations.len()*i] = 0.0;
    }
    minimum_distances
}

pub fn floyds(graph: HashMap<String, Vec<Edge>>, locations: Vec<Location>) -> Vec<f64> {
    let mut minimum_distances = fill_matrix_for_floyds(locations.clone(), graph);
    for k in 1..locations.len() {
        for i in 1..locations.len() {
            for j in 1..locations.len() {
                if minimum_distances[i+locations.len()*j] > minimum_distances[i+locations.len()*k] + minimum_distances[k+locations.len()*j] {
                    minimum_distances[i+locations.len()*j] = minimum_distances[i+locations.len()*k] + minimum_distances[k+locations.len()*j] 
                }
            }
        }
    }
    minimum_distances
}
