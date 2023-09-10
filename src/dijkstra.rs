// Not what i was looking for
use std::collections::HashMap;
use crate::Edge;

pub fn dijkstra(graph : HashMap<String, Vec<Edge>>, starting_node: &str, target_node: &str) -> (f64, Vec<String>) {
    let mut unvisited_nodes = Vec::from_iter(graph.keys());
    let mut distances: HashMap<String, f64> = HashMap::new();
    let mut prev: HashMap<String, String> = HashMap::new();
    for i in &mut distances {
        *i.1 = f64::MAX;
    }
    distances.insert(starting_node.to_string(), 0.0);
    let mut current_node = starting_node.to_string(); let mut next_node = starting_node.to_string();

    while unvisited_nodes.len() > 1 {
        unvisited_nodes.retain(|&node| node != &current_node);
        // unvisited_nodes.swap_remove(unvisited_nodes.clone().iter().position(|z| **z == current_node).unwrap());
        /*
        for i in 0..unvisited_nodes.len() {
            if unvisited_nodes[i] == &current_node {
                unvisited_nodes.remove(i);
                println!("{}", current_node);
                break;
            }
            println!("{}", current_node);
        } */
        println!("{}", current_node);
        let edges: Vec<Edge> = graph.get(&current_node).unwrap().to_vec();
        let mut closest_node_distance = f64::MAX;
        for edge in edges {
            if unvisited_nodes.iter().any(|&i| i==&edge.to) {
                if (distances.get(&current_node).unwrap_or(&f64::MAX) + edge.weight) < *distances.get(&edge.to).unwrap_or(&f64::MAX) {
                    distances.insert(edge.to.as_str().to_string(), distances.get(&current_node).unwrap() + edge.weight);
                    prev.insert(edge.to.as_str().to_string(), current_node.to_string());
                }
                if edge.weight < closest_node_distance && unvisited_nodes.contains(&&edge.to){
                    closest_node_distance = edge.weight;
                    next_node = edge.to;
                }
            }
        }
        current_node = next_node.clone();
    }
    (*distances.get(target_node).unwrap(), work_path(prev, target_node.to_string(), starting_node.to_string()))
}

// Stolen from wikepedia
pub fn work_path(prev: HashMap<String,String>, target_node: String, starting_node: String) -> Vec<String> {
    let mut s: Vec<String> = Vec::new();
    let mut u = target_node.as_str();
    while prev.get(u).is_some() && u!=starting_node {
        s.insert(0, u.to_string());
        u = prev.get(u).unwrap();
    }
    s.insert(0, "AbuDhabi".to_string());
    s
}

