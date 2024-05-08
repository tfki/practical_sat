use std::collections::HashMap;

use solver::{AtMostOneStrategy, ExactlyKStrategy, ipasir, LitValue, SatProblemResult, Solver, SolveResult};
use solver::literal::Lit;

use crate::{CellContent, Input, Solution};

pub fn solve(input: Input) -> SatProblemResult<Solution> {
    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::<String, Lit>::new();

    for x in 0..input.width {
        for y in 0..input.height {
            let mut lits = vec![];

            lits.push(*var_map.entry(format!("{x}/{y} = {:?}", CellContent::I)).or_insert(solver.new_lit()));
            lits.push(*var_map.entry(format!("{x}/{y} = {:?}", CellContent::T)).or_insert(solver.new_lit()));
            lits.push(*var_map.entry(format!("{x}/{y} = {:?}", CellContent::L)).or_insert(solver.new_lit()));
            lits.push(*var_map.entry(format!("{x}/{y} = {:?}", CellContent::S)).or_insert(solver.new_lit()));
            lits.push(*var_map.entry(format!("{x}/{y} = {:?}", CellContent::O)).or_insert(solver.new_lit()));

            solver.at_most_one(AtMostOneStrategy::Pairwise, &lits);
        }
    }

    for t in CellContent::all_values() {
        let mut anchors = vec![];
        for x in 0..input.width.saturating_sub(t.dimensions().0.saturating_sub(1)) {
            for y in 0..input.height.saturating_sub(t.dimensions().1.saturating_sub(1)) {
                let anchor = *var_map.entry(format!("{x}/{y} is {t:?} anchor")).or_insert(solver.new_lit());
                anchors.push(anchor);

                for (offset_x, offset_y) in t.offsets() {
                    let lit = var_map[&format!("{}/{} = {t:?}", x + offset_x, y + offset_y)];
                    solver.add_clause([-anchor, lit]);
                }
            }
        }

        for x in 0..input.width.saturating_sub(t.dimensions().0.saturating_sub(1)) {
            for y in 0..input.height.saturating_sub(t.dimensions().1.saturating_sub(1)) {
                let anchor = var_map[&format!("{x}/{y} is {t:?} anchor")];

                for (offset_x, offset_y) in t.excluded_other_anchor_offsets() {
                    if let Some(other_anchor) = var_map.get(&format!("{}/{} is {t:?} anchor", x as i32 + offset_x, y as i32 + offset_y)) {
                        solver.add_clause([-anchor, -*other_anchor]);
                    }
                }
            }
        }

        solver.exactly_k(ExactlyKStrategy::SequentialCounter, &anchors,
                         match t {
                             CellContent::I => input.num_i,
                             CellContent::O => input.num_o,
                             CellContent::S => input.num_s,
                             CellContent::T => input.num_t,
                             CellContent::L => input.num_l,
                         });
    }

    match solver.solve() {
        SolveResult::Sat => {
            let mut solution = Solution::new(input.width, input.height);

            let mut keys = var_map.keys().cloned().collect::<Vec<String>>();
            keys.sort();
            for key in keys {
                println!("{key} is {:?}", solver.val(var_map[&key]))
            }

            for x in 0..input.width {
                for y in 0..input.height {
                    for value in CellContent::all_values() {
                        if let Some(lit) = var_map.get(&format!("{x}/{y} is {value:?} anchor")) {
                            if LitValue::True == solver.val(*lit) {
                                for (x_offset, y_offset) in value.offsets() {
                                    solution.grid[(x + x_offset) as usize][(y + y_offset) as usize] = Some(value);
                                }
                            }
                        }
                    }
                }
            }

            SatProblemResult::Sat(solution)
        }
        SolveResult::Unsat => SatProblemResult::Unsat,
    }
}
