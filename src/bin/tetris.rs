use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let input = tetris::Input {
        height: args[1].parse().unwrap(),
        width: args[2].parse().unwrap(),
        num_i: args[3].parse().unwrap(),
        num_t: args[4].parse().unwrap(),
        num_s: args[5].parse().unwrap(),
        num_l: args[6].parse().unwrap(),
        num_o: args[7].parse().unwrap(),
    };
    println!("{}", tetris::naive2::gen_dimacs(input));
}
