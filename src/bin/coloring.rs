use std::env;

use practical_sat::ex1::coloring::{bitvec_incremental, FindKResult};
use practical_sat::ex1::graph::Graph;
use practical_sat::util::Timer;

fn main() {
    let graph_path = env::args().last().unwrap();
    let graph = Graph::parse_dimacs(graph_path);
    match bitvec_incremental::find_k(graph, Timer::new_infinite()) {
        FindKResult::Found(k) => println!("{k}"),
        x => println!("{x:?}"),
    }
}
