use std::env;

use practical_sat::ex1::coloring::{FindKResult, hybrid};
use practical_sat::ex1::coloring::graph::Graph;
use practical_sat::util::Timer;

fn main() {
    let graph_path = env::args().last().unwrap();
    let graph = Graph::parse_dimacs(graph_path);
    match hybrid::find_k(graph, Timer::new_infinite()) {
        FindKResult::Found(k) => println!("{k}"),
        x => println!("{x:?}"),
    }
}
