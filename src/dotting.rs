/// This file contains methods that generate the GUI graph

use crate::parse::Location;
use graphviz_rust::cmd::*;
use graphviz_rust::exec_dot;

/// Given the dot_graph as a string this function utilises graphviz
/// installed on the machine to produce an svg
///
/// Args:
///     dot_graph: String - the dot graph as a string
pub fn generate_graph(dot_graph: String) {
    let _ = exec_dot(
        dot_graph,
        vec![
            CommandArg::Layout(Layout::Neato),
            CommandArg::Format(Format::Svg),
            CommandArg::Output("/tmp/graph.svg".to_string())
        ]
    );
}

/// Taking in the locations and the worked out path this function produces a graphviz 
/// string/configuratin that can be used to make a graph
///
/// Args:
///     Locations: Vec<Location> - A vector containing all of the F1 locations
///     path: Vec<String> - the path that has been worked out
pub fn generate_dot(locations: Vec<Location>, path_1: Vec<String>, path_2: Vec<String>) -> String {
    let mut g: String = String::from("digraph main {\n");
    for location in locations {
        let position = calculate_pos(location.clone());
        g.push_str(format!("    \"{}\" [pos=\"{},{}!\", shape=plain]\n", location.location, position.0, position.1).as_str());
    }
    for i in 0..path_1.len()-1 {
        g.push_str(format!("    \"{}\" -> \"{}\"[color=\"red\"]\n", path_1[i], path_1[i+1]).as_str());
    }
    for i in 0..path_2.len()-1 {
        g.push_str(format!("    \"{}\" -> \"{}\"[color=\"blue\"]\n", path_2[i], path_2[i+1]).as_str());
    }
    g.push('}');
    g
}

/// This function shrinks the graph so it can fit more easily on the screen,
/// it does this by moving locations closer together.
/// Currently it aims for a 3 inch height and a 6 inch width
/// Args:
///     location: Location - a location
const DIVIDER: f64 = 5.0;
fn calculate_pos(location: Location) -> (f64, f64) {
    let horizontal: f64 = (location.lon + 180.0)/DIVIDER;
    let vertical: f64 = (location.lat + 90.0)/DIVIDER;
    (horizontal, vertical)
}
