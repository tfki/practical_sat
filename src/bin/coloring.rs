use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use practical_sat::solver::Solver;

struct Edge(u32, u32);

struct Graph {
    edges: Vec<Edge>,
}

fn main() {
    let graph_path = env::args().last().unwrap();

    let reader = BufReader::new(File::open(graph_path).unwrap());
    let lines = reader.lines();

    let mut graph = Graph { edges: vec![] };

    for line in lines {
        let line = line.unwrap();

        if line.starts_with('p') {
            let mut split = line.split_ascii_whitespace();

            // ignore 'p'
            split.next().unwrap();

            // ignore 'edge'
            split.next().unwrap();

            let _num_vertices = split.next().unwrap().parse::<u32>().unwrap();
            let num_edges = split.next().unwrap().parse::<u32>().unwrap();

            graph.edges.reserve(num_edges as usize);
        } else if line.starts_with('e') {
            let mut split = line.split_ascii_whitespace();

            // ignore 'e'
            split.next().unwrap();

            let node_1 = split.next().unwrap().parse::<u32>().unwrap();
            let node_2 = split.next().unwrap().parse::<u32>().unwrap();

            graph.edges.push(Edge(node_1, node_2));
        }
    }
    
    
    let mut solver = Solver::new();
    
    
    todo!()
}
