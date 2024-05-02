use std::env;
use practical_sat::ex1::sudoku::{smart_one_hot, Sudoku};

fn main() {
    let no_opt_trivial = env::args().any(|arg| arg == "--no-opt");
    let sudoku_path = env::args().last().unwrap();
    let input = Sudoku::parse(sudoku_path);
    
    println!("{}", smart_one_hot::gen_dimacs(&input, no_opt_trivial));
}
