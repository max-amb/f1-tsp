#![allow(unused_macros)]
mod parse;
mod distance;
mod graphing;
mod dotting;
mod nearest_neighbour;
mod simulated_annealing;
use std::collections::HashMap;
use parse::Location;

/// The edge struct
/// to - the location the edge is going to
/// weight - the distance to that location
#[derive(Clone,Debug,PartialEq)]
pub struct Edge {
    to: String,
    weight: f64,
}

/// First it gets the locations from the JSON
/// Then it creates the graph from create_graph
/// Next, using nearest_neighbour it calculates the fastest route round
/// Finally, using the path it generates a graph to display to the user
fn main() {
    let locations = parse::parse_json().unwrap();
    let graph: HashMap<String, Vec<Edge>> = graphing::create_graph(locations.clone());
    let path_nn = nearest_neighbour::nn(&graph, &locations, String::from("Zandvoort"));
    let path = simulated_annealing::simulated_annealing(&graph);
    println!("Nearest neighbour: {}, Simulated Annealing: {}", simulated_annealing::cost_func(&graph, &path_nn), simulated_annealing::cost_func(&graph, &path));
    println!("Nearest neighbour: {:?}, Simulated Annealing: {:?}", path_nn, path);
    let dot_graph = dotting::generate_dot(locations, path, path_nn);
    dotting::generate_graph(dot_graph);
}
