use std::env;
use hidoku::Hidoku;
use hidoku::encodings::naive_no_preprocessor::solve;

fn main() {
    assert!(env::args().len() >= 2);
    let problem_string = env::args().nth(1).unwrap();
    let hidoku = Hidoku::from(problem_string.clone());
    let solution = solve(hidoku.clone());
    println!("{solution:#?}");
}
