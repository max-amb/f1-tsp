use std::collections::HashMap;
use rand::Rng;
use crate::Edge;

/// This function calculates the cost of a path given the graph (for weights)
/// and the path
pub fn cost_func(graph: &HashMap<String, Vec<Edge>>, path: &[String]) -> f64 {
    let mut cost: f64 = 0.0;
    for path_section in 0..path.len()-1 {
        let edges: &Vec<Edge> = match graph.get(&path[path_section]) {
            Some(edges) => edges,
            None => panic!("Location disappeared"),
        };
        cost += edges.iter().find(|&x| x.to == path[path_section+1]).unwrap().weight;
    }
    cost
}

/// This function generates the random rearrangements/mutations required in simulated annealing, it
/// randomly chooses between switching the direction of two paths and moving a bit of the path
fn rearrangement(path: &[String]) -> Vec<String> {
    let mut new_path: Vec<String> = path.to_vec();
    if rand::thread_rng().gen_range(0.0..=1.0) < 0.5 {
        // Switch path direction
        let location_in_path_to_switch: usize = rand::thread_rng().gen_range(0..path.len()-1);
        let tmp = new_path[location_in_path_to_switch].to_string();
        new_path[location_in_path_to_switch] = path[location_in_path_to_switch+1].to_string();
        new_path[location_in_path_to_switch+1] = tmp;
    } else {
        // Move path bit
        let location_in_path_to_switch: usize = rand::thread_rng().gen_range(0..path.len()-1);
        let mut bit_being_moved: Vec<String> = Vec::new();
        bit_being_moved.push(new_path.remove(location_in_path_to_switch));
        bit_being_moved.push(new_path.remove(location_in_path_to_switch));
        let location_to_put_it_in: usize = rand::thread_rng().gen_range(0..path.len()-1);
        new_path.insert(location_to_put_it_in, bit_being_moved.remove(1));
        new_path.insert(location_to_put_it_in, bit_being_moved.remove(0));
    }
    new_path 
}

/// This function calculates the starting temperature for our simulated annealing algorithm, it
/// does this by randomly rearranging a path and storing all of the different cost functions, and
/// then it returns the highest multiplied by a constant factor
fn calculating_initial_temp(graph: &HashMap<String, Vec<Edge>>) -> f64 {
    let mut costs: Vec<f64> = Vec::new();
    let initial_path: Vec<String> = graph.clone().into_keys().collect();
    costs.push(cost_func(graph, &initial_path));
    let mut new_path: Vec<String> = rearrangement(&initial_path);
    for i in 1..10 {
        costs.push(cost_func(graph, &new_path)-costs[i-1]);
        new_path = rearrangement(&new_path);
    }
    costs.sort_by(|a,b| a.total_cmp(b));
    costs[costs.len()-1]*0.2
}

/// This function performs most of the logic for simulated annealing as per numerical recipes
/// recommends
pub fn simulated_annealing(graph: &HashMap<String, Vec<Edge>>) -> Vec<String> {
    let mut temp: f64 = calculating_initial_temp(graph);
    let mut path: Vec<String> = graph.clone().into_keys().collect();
    loop {
        let mut succesful_in_row: i32 = 0;
        let mut singular_success: bool = false;
        for _ in 0..path.len()*100 {
            let new_path: Vec<String> = rearrangement(&path);
            let current_cost = cost_func(graph, &path);
            let new_cost = cost_func(graph, &new_path);
            if new_cost < current_cost {
                singular_success = true;
            }
            let prob_of_jump: f64 = ((current_cost-new_cost)/temp).exp();

            if rand::thread_rng().gen_range(0.0..1.0) < prob_of_jump {
                path = new_path;
                succesful_in_row += 1;
            }

            if succesful_in_row as usize >= path.len()*10 {
                break;
            }
        }
        temp *= 0.99;
        println!("{}: {}", temp, cost_func(graph, &path));
        if !singular_success {
            break;
        }
    }
    path
}
