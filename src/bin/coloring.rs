use std::env;
use practical_sat::ex1::graph::Graph;


fn main() {
    let graph_path = env::args().last().unwrap();
    let graph = Graph::parse_dimacs(graph_path);

    println!("{}", practical_sat::ex1::find_k(graph, u32::MAX).unwrap());
}
