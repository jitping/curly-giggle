use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

use crate::graph::Distance;
use crate::graph::Edge;
use crate::graph::Vertex;
use graph::Graph;
mod graph;
mod test;

fn read_file(path: &str) -> (usize, Vec<(usize, usize, usize)>, HashMap<String, usize>) {
    let mut first_line = true;
    let mut result: Vec<(usize, usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut nodes_list: HashMap<String, usize> = HashMap::new();
    let mut nodes_counter: usize = 0;
    let mut raw_edges: usize = 0;
    let mut z_rejected: usize = 0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        if first_line {
            first_line = false;
        } else {
            raw_edges += 1;
            let v: Vec<&str> = line_str.trim().split(',').collect();
            let mut x_str: String = v[4].to_string();
            x_str = x_str.trim_matches('"').to_string();
            let mut y_str: String = v[6].to_string();
            y_str = y_str.trim_matches('"').to_string();
            let date_time_1 = NaiveDateTime::parse_from_str(
                &v[2].trim_matches('"').to_string(),
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Not In Time Format");
            let date_time_2 = NaiveDateTime::parse_from_str(
                &v[3].trim_matches('"').to_string(),
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Not In Time Format");
            let z: i64 = date_time_2.signed_duration_since(date_time_1).num_seconds();
            if z <= 0 {
                z_rejected += 1;
                continue;
            };
            if !nodes_list.contains_key(&x_str) {
                nodes_list.insert(x_str.clone(), nodes_counter);
                nodes_counter += 1;
            }
            if !nodes_list.contains_key(&y_str) {
                nodes_list.insert(y_str.clone(), nodes_counter);
                nodes_counter += 1;
            }
            let x = nodes_list.get(&x_str).unwrap();
            let y = nodes_list.get(&y_str).unwrap();
            if x != y {
                result.push((*x, *y, z as usize));
            }
        }
    }
    println!("Unique number of nodes is {}", nodes_counter);
    println!("Number of edges, including duplicates is {}", raw_edges);
    println!(
        "Number of rejected edges because of negative duration is {}",
        z_rejected
    );
    result.sort();
    let path = "nodes.txt";
    let mut output = File::create(path).expect("failed to create file");
    for (name, node_id) in &nodes_list {
        write!(output, "Station Name:{} ID:{}\n", name, node_id).expect("failed to create file");
    }
    return (nodes_counter, result, nodes_list);
}

fn unique_edges(input: Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut unique_tuples: Vec<(usize, usize, usize)> = Vec::new();
    let mut tuples_list = HashMap::new();
    for i in 0..input.len() {
        let x: usize = input[i].0;
        let y: usize = input[i].1;
        if tuples_list.contains_key(&(x, y)) {
            continue;
        }
        let mut total_count: usize = 0;
        let mut total_value: usize = 0;
        for j in i..input.len() {
            if x == input[j].0 && y == input[j].1 {
                total_count += 1;
                total_value += input[j].2;
            }
            if input[j].0 > x {
                break;
            };
        }
        let z: usize = total_value / total_count;
        tuples_list.insert((x, y), z);
        unique_tuples.push((x, y, z))
    }
    let unique_num_edges: usize = unique_tuples.len();
    println!(
        "After finding average travelling time for duplicate edges,unique number of edges is {}",
        unique_num_edges
    );
    return unique_tuples;
}
fn seconds_to_minutes_and_seconds(total_seconds: usize) -> (usize, usize) {
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    (minutes, seconds)
}

fn write_vertex_shortest_paths(
    nodes: HashMap<String, usize>,
    calculated_length: Vec<Option<Distance>>,
    starting_node: String,
) {
    let mut path: String = "Cycling Time To Other Citibike Stations From ".to_string();
    path.push_str(&starting_node);
    path.push_str(".txt");
    let mut output = File::create(path).expect("failed to create file");
    write!(
        output,
        "All Distances Calculated From Citibike Station At {} \n",
        starting_node
    )
    .expect("failed to create file");
    for (name, node_id) in &nodes {
        let calculations = calculated_length[*node_id];
        match calculations {
            Some(seconds) => write!(
                output,
                "To {} travelling time is {} minutes {} seconds\n",
                name,
                seconds_to_minutes_and_seconds(seconds).0,
                seconds_to_minutes_and_seconds(seconds).1
            )
            .expect("failed to create file"),
            None => {
                write!(output, "To {} has no known path\n", name).expect("failed to create file")
            }
        }
    }
}

fn main() {
    let filename = String::from("202304-citibike-tripdata.csv"); //Insert Filename Here
    let start_location = String::from("Clinton St & Joralemon St"); //insert starting point here. A Txt file (nodes.txt) is generated by lines 70-74 showing the list of stations//
    let end_location = String::from("Grand St & Havemeyer St"); //insert ending point here.

    //Implement Graph//
    let original_data = read_file(&filename);
    let unique_data = unique_edges(original_data.1);
    let n = original_data.0;
    let edges: Vec<Edge> = unique_data;
    let graph = Graph::create_directed(n, &edges);

    //To Find Shortest Distance Between Two Points, Insert Values Above//
    let start: Vertex = *(original_data.2)
        .get(&start_location)
        .expect("Invalid Start Location");
    let end: usize = *(original_data.2)
        .get(&end_location)
        .expect("Invalid End Location");
    let (time_mins, time_secs) =
        seconds_to_minutes_and_seconds(graph.find_shortest_path(start, end));
    println!(
        "Shortest travelling time from {} to {} is {} minutes {} seconds",
        start_location, end_location, time_mins, time_secs
    );
    //To Find Distance From A Given Citibike Dock (start_location) To All Other Docks. Will Print Out A Formatted .txt file
    write_vertex_shortest_paths(
        original_data.2,
        graph.shortest_path_from_vertex(start),
        start_location,
    );
}
