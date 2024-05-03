use std::collections::HashMap;
use std::time::Duration;

use crate::solver::literal::Lit;
use crate::ex1::sudoku::{Cell, Sudoku};
use crate::SatProblemResult;
use crate::solver::{AtMostOneStrategy, dimacs_emitting, ipasir, LitValue, Solver, SolveWithTimeoutResult};

pub fn find_solution(sudoku: &Sudoku, timeout: Duration) -> SatProblemResult<Sudoku> {
    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::new();

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    let mut buffer = vec![];
    for (x, y) in cells.clone() {
        if let Cell::Occupied(num) = sudoku.cell(x, y) {
            let lit = *var_map.entry(format!("{x}/{y} = {num}")).or_insert(solver.new_lit());
            solver.add_clause([lit]);
        }

        buffer.clear();

        for val in vals.clone() {
            let lit = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(solver.new_lit());
            buffer.push(lit);
        }

        solver.at_least_one(&buffer);
        solver.at_most_one(AtMostOneStrategy::Pairwise, &buffer);
    }

    for col in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.col(col).enumerate().map(|(y, _)| {
                *var_map.entry(format!("{col}/{y} = {val}")).or_insert(solver.new_lit())
            }).collect::<Vec<Lit>>();

            solver.at_least_one(&lits);
        }
    }

    for row in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.row(row).enumerate().map(|(x, _)| {
                *var_map.entry(format!("{x}/{row} = {val}")).or_insert(solver.new_lit())
            }).collect::<Vec<Lit>>();

            solver.at_least_one(&lits);
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let x_offset = block_x * sudoku.n;
            let y_offset = block_y * sudoku.n;

            for val in vals.clone() {
                buffer.clear();
                for x in x_offset..(x_offset + sudoku.n) {
                    for y in y_offset..(y_offset + sudoku.n) {
                        let lit = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(solver.new_lit());
                        buffer.push(lit);
                    }
                }
                solver.at_least_one(&buffer);
                solver.at_most_one(AtMostOneStrategy::Pairwise, &buffer);
            }
        }
    }

    let mut sudoku_solution = sudoku.clone();

    match solver.solve_with_timeout(timeout) {
        SolveWithTimeoutResult::Sat => {
            for (x, y) in cells.clone() {
                for val in vals.clone() {
                    let lit = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(solver.new_lit());

                    if let LitValue::True = solver.val(lit) {
                        *sudoku_solution.cell_mut(x, y) = Cell::Occupied(val);
                        break;
                    }
                }
            }

            SatProblemResult::Sat(sudoku_solution)
        }
        SolveWithTimeoutResult::TimeoutReached => SatProblemResult::Timeout,
        SolveWithTimeoutResult::Unsat => SatProblemResult::Unsat,
    }
}

pub fn gen_dimacs(sudoku: &Sudoku) -> String {
    let mut solver = Solver::<dimacs_emitting::Solver>::new();
    let mut var_map = HashMap::new();

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    let mut buffer = vec![];
    for (x, y) in cells.clone() {
        if let Cell::Occupied(num) = sudoku.cell(x, y) {
            let lit = *var_map.entry(format!("{x}/{y} = {num}")).or_insert(solver.new_lit());
            solver.add_clause([lit]);
        }

        buffer.clear();

        for val in vals.clone() {
            let lit = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(solver.new_lit());
            buffer.push(lit);
        }

        solver.at_least_one(&buffer);
        solver.at_most_one(AtMostOneStrategy::Pairwise, &buffer);
    }

    for col in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.col(col).enumerate().map(|(y, _)| {
                let lit = *var_map.entry(format!("{col}/{y} = {val}")).or_insert(solver.new_lit());
                lit
            }).collect::<Vec<Lit>>();

            solver.at_least_one(&lits);
        }
    }

    for row in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.row(row).enumerate().map(|(x, _)| {
                let lit = *var_map.entry(format!("{x}/{row} = {val}")).or_insert(solver.new_lit());
                lit
            }).collect::<Vec<Lit>>();

            solver.at_least_one(&lits);
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let x_offset = block_x * sudoku.n;
            let y_offset = block_y * sudoku.n;

            for val in vals.clone() {
                buffer.clear();
                for x in x_offset..(x_offset + sudoku.n) {
                    for y in y_offset..(y_offset + sudoku.n) {
                        let lit = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(solver.new_lit());
                        buffer.push(lit);
                    }
                }
                solver.at_least_one(&buffer);
                solver.at_most_one(AtMostOneStrategy::Pairwise, &buffer);
            }
        }
    }

    for (key, value) in var_map {
        solver.implementation.add_comment(format!("{key} <=> {}", i32::from(value)));
    }

    solver.implementation.get_dimacs()
}
