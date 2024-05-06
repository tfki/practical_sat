use std::env;
use sudoku::{smart_one_hot, Sudoku};

fn main() {
    let sudoku_path = env::args().last().unwrap();
    let input = Sudoku::parse(sudoku_path);
    
    println!("{}", smart_one_hot::gen_dimacs(&input));
}
