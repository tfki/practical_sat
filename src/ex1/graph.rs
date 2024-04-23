use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Edge(pub u32, pub u32);

pub struct Graph {
    pub edges: Vec<Edge>,
    pub num_vertices: u32,
}

impl Graph {
    pub fn parse_dimacs(path: impl AsRef<Path>) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let lines = reader.lines();

        let mut graph = Graph { edges: vec![], num_vertices: 0 };

        for line in lines {
            let line = line.unwrap();

            if line.starts_with('p') {
                let mut split = line.split_ascii_whitespace();

                // ignore 'p'
                split.next().unwrap();

                // ignore 'edge'
                split.next().unwrap();

                let num_vertices = split.next().unwrap().parse::<u32>().unwrap();
                let num_edges = split.next().unwrap().parse::<u32>().unwrap();

                graph.edges.reserve(num_edges as usize);
                graph.num_vertices = num_vertices;
            } else if line.starts_with('e') {
                let mut split = line.split_ascii_whitespace();

                // ignore 'e'
                split.next().unwrap();

                let node_1 = split.next().unwrap().parse::<u32>().unwrap();
                let node_2 = split.next().unwrap().parse::<u32>().unwrap();

                graph.edges.push(Edge(node_1, node_2));
            }
        }
        graph
    }
}
