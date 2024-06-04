use std::env;

use sls::cnf::Cnf;
use sls::sls::{solve, Strategy};

fn main() {
    assert!(env::args().count() > 1);
    let path = env::args().last().unwrap();
    let cnf = Cnf::from_dimacs(path);

    match solve(cnf, Strategy::WalkSat, None) {
        None => println!("s UNKNOWN"),
        Some(assignment) => {
            println!("s SATISFIABLE");

            let mut line_length = 1_usize;
            print!("v");

            for (var, value) in assignment {
                let string = format!(" {}{}", if value { "" } else { "-" }, var.id);

                if line_length + string.len() > 4096 {
                    println!();
                    print!("v");
                    line_length = 1;
                }
                
                print!("{string}");
                line_length += string.len();
            }
        }
    }
}
