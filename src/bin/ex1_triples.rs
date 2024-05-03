use std::collections::HashMap;
use std::env;
use practical_sat::solver::literal::Lit;
use practical_sat::solver::{ipasir, Solver, SolveResult};

fn main() {
    let max = env::args().last().unwrap().parse::<u32>().unwrap();
    let mut var_map = HashMap::<String, Lit>::new();
    let mut solver = Solver::<ipasir::Solver>::new();
    let mut triples_count = 0;

    for n in 1..=max {
        for m in (n + 1)..=max {
            for k in 1..max {
                if gcd::binary_u32(m, n) != 1 || ((m % 2 == 1) && (n % 2 == 1)) {
                    continue;
                }

                let triple = triple(m, n, k);
                if triple.iter().any(|x| *x > max) {
                    break;
                } else {
                    triples_count += 1;
                    let a = *var_map.entry(format!("{:5} is colored", triple[0])).or_insert(solver.new_lit());
                    let b = *var_map.entry(format!("{:5} is colored", triple[1])).or_insert(solver.new_lit());
                    let c = *var_map.entry(format!("{:5} is colored", triple[2])).or_insert(solver.new_lit());

                    solver.add_clause([a, b, c]);
                    solver.add_clause([-a, -b, -c]);
                    println!("{triple:?}");
                }
            }
        }
    }
    println!("there are {triples_count} triples, {} clauses and {} variables", 2 * triples_count, var_map.len());
    
    match solver.solve() {
        SolveResult::Sat => {
            let mut keys = var_map.keys().collect::<Vec<&String>>();
            keys.sort();
            
            for key in keys {
                println!("{key} = {:?}", solver.val(var_map[key]));
            }
        }
        x => println!("{x:?}"),
    }
}

fn triple(m: u32, n: u32, k: u32) -> [u32; 3] {
    [k * (m.pow(2) - n.pow(2)), k * 2 * m * n, k * (m.pow(2) + n.pow(2))]
}
