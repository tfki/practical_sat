use std::collections::HashMap;

use crate::cnf::literal::{Literal, Variable};
use crate::ex1::sudoku::sudoku::{Cell, Sudoku};
use crate::solver::{Solver, SolveResult, VariableValue};

pub fn find_solution(sudoku: &Sudoku) -> Option<Sudoku> {
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
            solver.add_clause(&[Literal::new_pos(id)]);
        }

        buffer.clear();

        for val in vals.clone() {
            let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());
            buffer.push(Literal::new_pos(id));
        }

        solver.at_least_one(&buffer);
        solver.at_most_one_pairwise(&buffer);
    }

    for col in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.col(col).enumerate().map(|(y, _)| {
                let id = *var_map.entry(format!("{col}/{y} = {val}")).or_insert(allocator.next().unwrap());
                Literal::new_pos(id)
            }).collect::<Vec<Literal>>();

            solver.at_least_one(&lits);
        }
    }

    for row in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.row(row).enumerate().map(|(x, cell)| {
                let id = *var_map.entry(format!("{x}/{row} = {val}")).or_insert(allocator.next().unwrap());
                Literal::new_pos(id)
            }).collect::<Vec<Literal>>();

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
                        buffer.push(Literal::new_pos(id));
                    }
                }
                solver.at_least_one(&buffer);
                solver.at_most_one_pairwise(&buffer);
            }
        }
    }

    let mut sudoku_solution = sudoku.clone();

    match solver.solve() {
        SolveResult::Sat => {
            for (x, y) in cells.clone() {
                for val in vals.clone() {
                    let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());

                    if let VariableValue::True = solver.val(Variable::new(id)) {
                        *sudoku_solution.cell_mut(x, y) = Cell::Occupied(val);
                        break;
                    }
                }
            }

            return Some(sudoku_solution);
        }
        x => println!("{x:?}"),
    }

    None
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
            solver.add_clause(&[Literal::new_pos(id)]);
        }

        buffer.clear();

        for val in vals.clone() {
            let id = *var_map.entry(format!("{x}/{y} = {val}")).or_insert(allocator.next().unwrap());
            buffer.push(Literal::new_pos(id));
        }

        solver.at_least_one(&buffer);
        solver.at_most_one_pairwise(&buffer);
    }

    for col in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.col(col).enumerate().map(|(y, _)| {
                let id = *var_map.entry(format!("{col}/{y} = {val}")).or_insert(allocator.next().unwrap());
                Literal::new_pos(id)
            }).collect::<Vec<Literal>>();

            solver.at_least_one(&lits);
        }
    }

    for row in one_axis_coords.clone() {
        for val in vals.clone() {
            let lits = sudoku.row(row).enumerate().map(|(x, _)| {
                let id = *var_map.entry(format!("{x}/{row} = {val}")).or_insert(allocator.next().unwrap());
                Literal::new_pos(id)
            }).collect::<Vec<Literal>>();

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
                        buffer.push(Literal::new_pos(id));
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
