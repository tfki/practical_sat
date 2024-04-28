use std::env;
use practical_sat::ex1::sudoku::naive_one_hot::gen_dimacs;
use practical_sat::ex1::sudoku::sudoku::Sudoku;

fn main() {
    let sudoku_path = env::args().last().unwrap();
    let input = Sudoku::parse(sudoku_path);
    
    println!("{}", gen_dimacs(&input));
}
