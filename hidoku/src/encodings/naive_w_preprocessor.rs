use std::collections::HashMap;

use solver::{AtMostOneStrategy, ExactlyKStrategy, ipasir, LitValue, SatProblemResult, Solver, SolveResult};
use solver::literal::Lit;

use crate::{get_possible_coords, Hidoku};

pub fn solve(mut hidoku: Hidoku) -> SatProblemResult<Hidoku> {
    let dimens = hidoku.dimens();
    let dimens_sq = dimens * dimens;

    let possible_coords =
        if let Some(vals) = get_possible_coords(&hidoku) {
            vals
        } else { return SatProblemResult::Unsat; };

    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::<String, Lit>::new();

    for (number, coords) in &possible_coords {
        let mut lits = vec![];

        for coord in coords {
            let lit = *var_map.entry(format!("{}/{} is {}", coord.x, coord.y, number)).or_insert(solver.new_lit());
            lits.push(lit);
        }

        if lits.len() > 1 {
            solver.exactly_k(ExactlyKStrategy::SequentialCounter, &lits, 1);
        }
    }

    for x in 0..dimens {
        for y in 0..dimens {
            let mut all_values = vec![];

            for value in 1..=dimens_sq {
                if let Some(lit) = var_map.get(&format!("{}/{} is {}", x, y, value)) {
                    all_values.push(*lit);
                }
            }

            if all_values.len() > 1 {
                solver.at_least_one(&all_values);
                solver.at_most_one(AtMostOneStrategy::Pairwise, &all_values);
            }
        }
    }

    for (number, coords) in &possible_coords {
        for coord in coords {
            let lit = *var_map.get(&format!("{}/{} is {}", coord.x, coord.y, number)).unwrap();

            // number in field implies its predecessor in one of neighboring fields
            if *number > 1 {
                let mut implication = vec![-lit];
                for neighbor in coord.neighbors_clipped(dimens) {
                    if let Some(other_lit) = var_map.get(&format!("{}/{} is {}", neighbor.x, neighbor.y, number - 1)) {
                        implication.push(*other_lit);
                    }
                }
                if implication.len() > 1 {
                    solver.add_clause(implication);
                }
            }
            if *number < dimens_sq as u32 {
                let mut implication = vec![-lit];
                for neighbor in coord.neighbors_clipped(dimens) {
                    if let Some(other_lit) = var_map.get(&format!("{}/{} is {}", neighbor.x, neighbor.y, number + 1)) {
                        implication.push(*other_lit);
                    }
                }
                if implication.len() > 1 {
                    solver.add_clause(implication);
                }
            }
        }
    }

    match solver.solve() {
        SolveResult::Sat => {
            // let mut keys = var_map.keys().collect::<Vec<&String>>();
            // keys.sort();
            //
            // for key in keys {
            //     println!("{key}: {:?}", solver.val(var_map[key]));
            // }

            for x in 0..dimens {
                for y in 0..dimens {
                    for value in 1..=dimens_sq {
                        if let Some(lit) = var_map.get(&format!("{}/{} is {}", x, y, value)) {
                            if solver.val(*lit) == LitValue::True {
                                hidoku.set(x, y, Some(value as u32));
                            }
                        }
                    }
                }
            }

            SatProblemResult::Sat(hidoku)
        }
        SolveResult::Unsat => SatProblemResult::Unsat,
    }
}
