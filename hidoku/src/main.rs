use hidoku::Hidoku;
use hidoku::encodings::naive_no_preprocessor::solve;

fn main() {
    let problem_path = "assets/hidoku_3x3_1";
    let problem_string = std::fs::read_to_string(problem_path).unwrap();
    let hidoku = Hidoku::from(problem_string.clone());
    let solution = solve(hidoku.clone());
    println!("{solution:#?}");
}



