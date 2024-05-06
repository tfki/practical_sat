use std::env;
use coloring::graph::Graph;
use coloring::{FindKResult, one_hot_incremental};
use solver::timer::Timer;


fn main() {
    let graph_path = env::args().last().unwrap();
    let graph = Graph::parse_dimacs(graph_path);
    match one_hot_incremental::find_k(graph, Timer::new_infinite()) {
        FindKResult::Found(k) => println!("{k}"),
        x => println!("{x:?}"),
    }
}
