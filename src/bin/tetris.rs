use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let input = tetris::Input{
        width: args[1].parse().unwrap(),
        height: args[2].parse().unwrap(),
        num_i: args[3].parse().unwrap(),
        num_t: args[4].parse().unwrap(),
        num_l: args[5].parse().unwrap(),
        num_s: args[6].parse().unwrap(),
        num_o: args[7].parse().unwrap(),
    };
    tetris::naive::solve(input);
}
