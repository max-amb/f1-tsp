use crate::Edge;
use crate::Location;
use crate::distance::calculate;
use std::collections::HashMap;

/// This macro is used to add an edge to the graph
macro_rules! add_edge{
    ($graph: expr, $from: expr, $to: expr, $weight: expr) => {
        $graph.entry($from).and_modify(|v| v.push(Edge { to: $to, weight: $weight } ))
    };
}

/// This function creates the graph from a vector of locations (taken from the Json)
/// Args:
///     locations: Vec<Location> - the locations
pub fn create_graph(locations: Vec<Location>) -> HashMap<String, Vec<Edge>> {
    let mut graph: HashMap<String, Vec<Edge>> = HashMap::new();
    for i in &locations {
        graph.insert(i.location.clone(), Vec::new());
    }
    add_edges(locations, graph.clone())
}

/// Given the locations and the graph, this function add's the edges, calculating their lengths
/// It initially creates an edge between every node, with the weight as the distance
fn add_edges(locations: Vec<Location>, graph: HashMap<String, Vec<Edge>>) -> HashMap<String, Vec<Edge>> {
    let mut new_graph = graph;
    for i in 0..locations.len() {
        for j in 0..locations.len() {
            let mut already_in: bool = false;
            let distance =  calculate(locations[i].lon.to_radians(), locations[i].lat.to_radians(), locations[j].lon.to_radians(), locations[j].lat.to_radians());
            for x in new_graph.values() {
                if x.contains(&Edge { to: locations[j].location.clone(), weight: distance}) {
                    already_in = true;
                }
            }
            if distance != 0.0 && !already_in {
                add_edge!(new_graph, locations[i].location.clone(), locations[j].location.clone(), distance);
            }
        }
    }
    new_graph
}

