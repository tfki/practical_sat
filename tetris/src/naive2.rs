use std::collections::HashMap;

use solver::{ExactlyKStrategy, ipasir, LitValue, SatProblemResult, Solver, SolveResult};
use solver::literal::Lit;

use crate::{CellContent, Input, Solution};

pub fn solve(input: Input) -> SatProblemResult<Solution> {
    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::<String, Lit>::new();

    for t in CellContent::all_values() {
        let mut anchors = vec![];
        for x in 0..input.width.saturating_sub(t.dimensions().0.saturating_sub(1)) {
            for y in 0..input.height.saturating_sub(t.dimensions().1.saturating_sub(1)) {
                let anchor = *var_map.entry(format!("{x}/{y} is {t:?} anchor")).or_insert(solver.new_lit());
                anchors.push(anchor);
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

    for t in CellContent::all_values() {
        for x in 0..(input.width.saturating_sub(t.dimensions().0.saturating_sub(1)) as i32) {
            for y in 0..(input.height.saturating_sub(t.dimensions().1.saturating_sub(1)) as i32) {
                let anchor = *var_map.entry(format!("{x}/{y} is {t:?} anchor")).or_insert(solver.new_lit());

                match t {
                    CellContent::I => {
                        for excluded_i_anchor_offsets in [(0, 1), (0, 2), (0, 3)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_i_anchor_offsets.0, y + excluded_i_anchor_offsets.1, CellContent::I)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_t_anchor_offsets in
                        [(0, 0), (0, 1), (0, 2), (0, 3), (-1, 1), (-1, 2), (-1, 3), (-2, 1), (-2, 2), (-2, 3)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_t_anchor_offsets.0, y + excluded_t_anchor_offsets.1, CellContent::T)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_s_anchor_offsets in
                        [(0, 0), (0, 1), (0, 2), (-1, 1), (-1, 2), (-1, 3), (-2, 1), (-2, 2), (-2, 3)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_s_anchor_offsets.0, y + excluded_s_anchor_offsets.1, CellContent::S)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_l_anchor_offsets in
                        [(0, 0), (0, 1), (0, 2), (0, 3), (-1, 1), (-1, 2), (-2, 1), (-2, 2)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_l_anchor_offsets.0, y + excluded_l_anchor_offsets.1, CellContent::L)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_o_anchor_offsets in
                        [(0, 0), (0, 1), (0, 2), (0, 3), (-1, 1), (-1, 2), (-1, 3)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_o_anchor_offsets.0, y + excluded_o_anchor_offsets.1, CellContent::O)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }
                    }
                    CellContent::O => {
                        for excluded_i_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_i_anchor_offsets.0, y + excluded_i_anchor_offsets.1, CellContent::I)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_t_anchor_offsets in
                        [(0, 0), (0, 1), (1, 0), (1, 1), (-1, 1), (-2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_t_anchor_offsets.0, y + excluded_t_anchor_offsets.1, CellContent::T)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_s_anchor_offsets in
                        [(0, 0), (0, 1), (1, 0), (-1, 1), (-2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_s_anchor_offsets.0, y + excluded_s_anchor_offsets.1, CellContent::S)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_l_anchor_offsets in
                        [(0, 0), (0, 1), (1, 0), (1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_l_anchor_offsets.0, y + excluded_l_anchor_offsets.1, CellContent::L)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_o_anchor_offsets in
                        [(0, 1), (1, 0), (1, 1), (-1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_o_anchor_offsets.0, y + excluded_o_anchor_offsets.1, CellContent::O)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }
                    }
                    CellContent::S => {
                        for excluded_i_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_i_anchor_offsets.0, y + excluded_i_anchor_offsets.1, CellContent::I)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_t_anchor_offsets in
                        [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (-1, 1), (-2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_t_anchor_offsets.0, y + excluded_t_anchor_offsets.1, CellContent::T)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_s_anchor_offsets in [(0, 1), (1, 0), (-1, 1), (-2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_s_anchor_offsets.0, y + excluded_s_anchor_offsets.1, CellContent::S)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_l_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (-2, 0)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_l_anchor_offsets.0, y + excluded_l_anchor_offsets.1, CellContent::L)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_o_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (-1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_o_anchor_offsets.0, y + excluded_o_anchor_offsets.1, CellContent::O)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }
                    }
                    CellContent::T => {
                        for excluded_i_anchor_offsets in [(0, 0), (1, 0), (1, 1), (2, 0)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_i_anchor_offsets.0, y + excluded_i_anchor_offsets.1, CellContent::I)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_t_anchor_offsets in [(0, 1), (1, 0), (1, 1), (2, 0), (-1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_t_anchor_offsets.0, y + excluded_t_anchor_offsets.1, CellContent::T)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_s_anchor_offsets in [(0, 0), (0, 1), (1, 0), (-1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_s_anchor_offsets.0, y + excluded_s_anchor_offsets.1, CellContent::S)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_l_anchor_offsets in [(0, 0), (1, 0), (1, 1), (2, 0)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_l_anchor_offsets.0, y + excluded_l_anchor_offsets.1, CellContent::L)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_o_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_o_anchor_offsets.0, y + excluded_o_anchor_offsets.1, CellContent::O)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }
                    }
                    CellContent::L => {
                        for excluded_i_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_i_anchor_offsets.0, y + excluded_i_anchor_offsets.1, CellContent::I)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_t_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 1), (-1, 1), (-2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_t_anchor_offsets.0, y + excluded_t_anchor_offsets.1, CellContent::T)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_s_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (-1, 1), (-2, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_s_anchor_offsets.0, y + excluded_s_anchor_offsets.1, CellContent::S)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_l_anchor_offsets in [(0, 1), (1, 0), (1, 1), (2, 0), (2, 1), (-2, 0)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_l_anchor_offsets.0, y + excluded_l_anchor_offsets.1, CellContent::L)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }

                        for excluded_o_anchor_offsets in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1), (-1, 1)] {
                            if let Some(other_anchor) = var_map.get(&format!("{}/{} is {:?} anchor", x + excluded_o_anchor_offsets.0, y + excluded_o_anchor_offsets.1, CellContent::O)) {
                                solver.add_clause([-anchor, -*other_anchor]);
                            }
                        }
                    }
                }
            }
        }
    }

    match solver.solve() {
        SolveResult::Sat => {
            let mut solution = Solution::new(input.width, input.height);

            // let mut keys = var_map.keys().cloned().collect::<Vec<String>>();
            // keys.sort();
            // for key in keys {
            //     println!("{key} is {:?}", solver.val(var_map[&key]))
            // }

            for x in 0..input.width {
                for y in 0..input.height {
                    for value in CellContent::all_values() {
                        let key = &format!("{x}/{y} is {value:?} anchor");
                        if let Some(lit) = var_map.get(key) {
                            if LitValue::True == solver.val(*lit) {
                                for (x_offset, y_offset) in value.offsets() {
                                    debug_assert!(solution.grid[(x + x_offset) as usize][(y + y_offset) as usize].is_none());
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
