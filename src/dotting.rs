use crate::Edge;
use crate::parse::Location;
use std::collections::HashMap;
use graphviz_rust::cmd::*;
use graphviz_rust::exec_dot;
pub fn generate_graph(dot_graph: String) {
    /*
    let g: Graph = parse(dot_graph.as_str()).unwrap();
    let _ = exec(
        g,
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("/home/max/Pictures/graph.svg".to_string()),
        ],
    ); */
    let _ = exec_dot(dot_graph, vec![CommandArg::Layout(Layout::Neato), CommandArg::Format(Format::Svg), CommandArg::Output("/tmp/graph.svg".to_string())]);
}

pub fn generate_dot(graph : HashMap<String, Vec<Edge>>, locations: Vec<Location>, path: Vec<String>) -> String {
    let mut g: String = String::from("digraph main {\n");
    for location in locations {
        let position = calculate_pos(location.clone());
        g.push_str(format!("    \"{}\" [pos=\"{},{}!\", shape=plain]\n", location.location, position.0, position.1).as_str());
    }
    for i in 0..path.len()-1 {
        g.push_str(format!("    \"{}\" -> \"{}\"[color=\"red\"]\n", path[i], path[i+1]).as_str());
    }
    g.push_str("}");
    g
}

// 3 inch height, 6 inch width
const DIVIDER: f64 = 5.0;
fn calculate_pos(location: Location) -> (f64, f64) {
    let horizontal: f64 = (location.lon + 180.0)/DIVIDER;
    let vertical: f64 = (location.lat + 90.0)/DIVIDER;
    (horizontal, vertical)
}
