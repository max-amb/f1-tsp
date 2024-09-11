#![allow(unused_macros)]
mod parse; mod distance; mod graphing; mod dotting; mod nearest_neighbour;
use std::collections::HashMap;
use parse::Location;

#[derive(Clone,Debug,PartialEq)]
pub struct Edge {
    to: String,
    weight: f64,
}

fn main() {
    let locations = parse::parse_json().unwrap();
    let graph: HashMap<String, Vec<Edge>> = graphing::create_graph(locations.clone());
    let path = nearest_neighbour::nn(graph.clone(), locations.clone(), "Zandvoort".to_string());
//    let (_, path) = dijkstra::dijkstra(graph.clone(), "Abu Dhabi", "Zandvoort");
//    let minimum_distances = wfi::floyds(graph, locations);
//    println!("{:?}", minimum_distances);
    let dot_graph = dotting::generate_dot(graph, locations, path);
    dotting::generate_graph(dot_graph);
}

/*
macro_rules! read_edge_weight {
    ($graph: expr, $key: expr, $to: expr) => {
        $graph.get(&$key).unwrap().get($graph.get(&$key).unwrap().iter().position(|v| v.to == $to ).unwrap()).unwrap().weight
    }
}
macro_rules! read_edge_weight_index {
    ($graph: expr, $key: expr, $index: expr) => {
        $graph.get(&$key).unwrap().get($index).unwrap().weight
    };
}
*/
