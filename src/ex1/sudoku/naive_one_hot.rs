use std::collections::HashMap;

use crate::cnf::literal::Lit;
use crate::ex1::sudoku::sudoku::{Cell, Sudoku};
use crate::SatProblemResult;
use crate::solver::{LitValue, Solver, SolveResult};
use crate::util::Timer;

pub fn find_solution(sudoku: &Sudoku, timer: Timer) -> SatProblemResult<Sudoku> {
    let mut solver = Solver::new();
    let mut var_map = HashMap::new();
    let mut allocator = 1..;

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    let mut buffer = vec![];
    for (x, y) in cells.clone() {
        if let Cell::Occupied(num) = sudoku.cell(x, y) {
            let id = *var_map.entry(format!("{x}/{y} = {num}")).or_insert(allocator.next().unwrap());
            solver.add_clause(&[Lit::new(id)]);
        }

        buffer.clear();

        for val in vals.clone() {
            let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());
            buffer.push(Lit::new(id));
        }

        solver.at_least_one(&buffer);
        solver.at_most_one_pairwise(&buffer);
    }

    for col in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.col(col).enumerate().map(|(y, _)| {
                let id = *var_map.entry(format!("{col}/{y} = {val}")).or_insert(allocator.next().unwrap());
                Lit::new(id)
            }).collect::<Vec<Lit>>();

            solver.at_least_one(&lits);
        }
    }

    for row in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.row(row).enumerate().map(|(x, _)| {
                let id = *var_map.entry(format!("{x}/{row} = {val}")).or_insert(allocator.next().unwrap());
                Lit::new(id)
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
                        let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());
                        buffer.push(Lit::new(id));
                    }
                }
                solver.at_least_one(&buffer);
                solver.at_most_one_pairwise(&buffer);
            }
        }
    }

    let mut sudoku_solution = sudoku.clone();

    solver.set_terminate(move || timer.has_finished());
    match solver.solve() {
        SolveResult::Sat => {
            for (x, y) in cells.clone() {
                for val in vals.clone() {
                    let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());

                    if let LitValue::True = solver.val(Lit::new(id)) {
                        *sudoku_solution.cell_mut(x, y) = Cell::Occupied(val);
                        break;
                    }
                }
            }

            SatProblemResult::Sat(sudoku_solution)
        }
        SolveResult::Interrupted => SatProblemResult::Timeout,
        SolveResult::Unsat => SatProblemResult::Unsat,
    }
}

pub fn gen_dimacs(sudoku: &Sudoku) -> String {
    let mut solver = crate::dimacs_emitting_solver::Solver::new();
    let mut var_map = HashMap::new();
    let mut allocator = 1..;

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    let mut buffer = vec![];
    for (x, y) in cells.clone() {
        if let Cell::Occupied(num) = sudoku.cell(x, y) {
            let id = *var_map.entry(format!("{x}/{y} = {num}")).or_insert(allocator.next().unwrap());
            solver.add_clause(&[Lit::new(id)]);
        }

        buffer.clear();

        for val in vals.clone() {
            let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());
            buffer.push(Lit::new(id));
        }

        solver.at_least_one(&buffer);
        solver.at_most_one_pairwise(&buffer);
    }

    for col in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.col(col).enumerate().map(|(y, _)| {
                let id = *var_map.entry(format!("{col}/{y} = {val}")).or_insert(allocator.next().unwrap());
                Lit::new(id)
            }).collect::<Vec<Lit>>();

            solver.at_least_one(&lits);
        }
    }

    for row in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.row(row).enumerate().map(|(x, _)| {
                let id = *var_map.entry(format!("{x}/{row} = {val}")).or_insert(allocator.next().unwrap());
                Lit::new(id)
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
                        let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());
                        buffer.push(Lit::new(id));
                    }
                }
                solver.at_least_one(&buffer);
                solver.at_most_one_pairwise(&buffer);
            }
        }
    }

    for (key, value) in var_map {
        solver.add_comment(format!("{key} <=> {value}"));
    }

    solver.get_dimacs()
}
