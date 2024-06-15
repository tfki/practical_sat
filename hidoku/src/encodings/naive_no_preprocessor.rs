use std::collections::HashMap;

use solver::{ExactlyKStrategy, ipasir, LitValue, SatProblemResult, Solver, SolveResult};
use solver::literal::Lit;

use crate::{Coord, Hidoku};

pub fn solve(mut hidoku: Hidoku) -> SatProblemResult<Hidoku> {
    let dimens = hidoku.dimens();
    let dimens_sq = dimens * dimens;

    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::<String, Lit>::new();

    for value in 1..=dimens_sq {
        let mut all_positions = vec![];

        for x in 0..dimens {
            for y in 0..dimens {
                all_positions.push(*var_map.entry(format!("{}/{} is {}", x, y, value)).or_insert(solver.new_lit()));
            }
        }
        solver.exactly_k(ExactlyKStrategy::SequentialCounter, &all_positions, 1);
    }

    for x in 0..dimens {
        for y in 0..dimens {
            let mut all_values = vec![];

            for value in 1..=dimens_sq {
                all_values.push(*var_map.get(&format!("{}/{} is {}", x, y, value)).unwrap());
            }

            solver.exactly_k(ExactlyKStrategy::SequentialCounter, &all_values, 1)
        }
    }

    for value in 1..=dimens_sq {
        for x in 0..dimens {
            for y in 0..dimens {
                let lit = *var_map.get(&format!("{}/{} is {}", x, y, value)).unwrap();

                if value > 1 {
                    let mut implication = vec![-lit];
                    for neighbor in (Coord { x, y }).neighbors_clipped(dimens) {
                        if hidoku.get(neighbor.x, neighbor.y).is_none() || hidoku.get(neighbor.x, neighbor.y).unwrap() == (value - 1) as u32 {
                            implication.push(*var_map.get(&format!("{}/{} is {}", neighbor.x, neighbor.y, value - 1)).unwrap());
                        }
                    }
                    if implication.len() > 1 {
                        solver.add_clause(implication);
                    }
                }
                if value < dimens_sq {
                    let mut implication = vec![-lit];
                    for neighbor in (Coord { x, y }).neighbors_clipped(dimens) {
                        if hidoku.get(neighbor.x, neighbor.y).is_none() || hidoku.get(neighbor.x, neighbor.y).unwrap() == (value + 1) as u32 {
                            implication.push(*var_map.get(&format!("{}/{} is {}", neighbor.x, neighbor.y, value + 1)).unwrap());
                        }
                    }
                    if implication.len() > 1 {
                        solver.add_clause(implication);
                    }
                }
            }
        }
    }

    for x in 0..dimens {
        for y in 0..dimens {
            if let Some(value) = hidoku.get(x, y) {
                let lit = *var_map.get(&format!("{}/{} is {}", x, y, value)).unwrap();
                solver.add_clause([lit]);
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
