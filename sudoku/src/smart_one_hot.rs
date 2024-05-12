use std::collections::HashMap;
use std::time::Duration;

use solver::{AtLeastKStrategy, AtMostOneStrategy, dimacs_emitting, ipasir, LitValue, SatProblemResult, Solver, SolverImpl, SolveWithTimeoutResult};
use solver::literal::Lit;

use crate::{Cell, Sudoku};

fn propagate_occupied_cells(sudoku: &Sudoku, grid: &mut Vec<Vec<Vec<bool>>>) {
    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            if let Cell::Occupied(num) = sudoku.cell(x, y) {
                grid[x as usize][y as usize].iter_mut().for_each(|cell| *cell = false);

                let block_x = x / sudoku.n;
                let block_y = y / sudoku.n;

                let x_offset = block_x * sudoku.n;
                let y_offset = block_y * sudoku.n;

                for x in x_offset..(x_offset + sudoku.n) {
                    for y in y_offset..(y_offset + sudoku.n) {
                        grid[x as usize][y as usize][*num as usize - 1] = false;
                    }
                }

                for x in 0..sudoku.n.pow(2) {
                    grid[x as usize][y as usize][*num as usize - 1] = false;
                }
                for y in 0..sudoku.n.pow(2) {
                    grid[x as usize][y as usize][*num as usize - 1] = false;
                }
            }
        }
    }
}

fn make_trivial_choices(sudoku: &mut Sudoku, grid: &mut Vec<Vec<Vec<bool>>>) -> Option<Vec<String>> {
    let mut comments = vec![];

    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            if grid[x as usize][y as usize].iter().filter(|item| **item).count() == 1 {
                let (index, _) = grid[x as usize][y as usize].iter().enumerate().find(|(_, item)| **item).unwrap();
                *sudoku.cell_mut(x, y) = Cell::Occupied(index as u32 + 1);
                comments.push(format!("{x}/{y} is trivially {}", index + 1));
            }
        }
    }

    for col_i in 0..sudoku.n.pow(2) {
        for val in 1..=sudoku.n.pow(2) {
            let mut iter = grid[col_i as usize].iter().enumerate()
                .filter_map(|(row_i, col)| if col[val as usize - 1] { Some(row_i) } else { None });
            let first = iter.next();
            let more_than_one = iter.next().is_some();

            if first.is_some() && !more_than_one && !sudoku.col(col_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                grid[col_i as usize][first.unwrap()].iter_mut().for_each(|lit| *lit = false);
                grid[col_i as usize][first.unwrap()][val as usize - 1] = true;

                *sudoku.cell_mut(col_i, first.unwrap() as u32) = Cell::Occupied(val);
                comments.push(format!("{col_i}/{} is trivially {}", first.unwrap(), val));
            }
        }
    }

    for row_i in 0..sudoku.n.pow(2) {
        for val in 1..=sudoku.n.pow(2) {
            let mut iter = grid.iter()
                .map(move |col| &col[row_i as usize])
                .enumerate()
                .filter_map(|(col_i, cell)| if cell[val as usize - 1] { Some(col_i) } else { None });
            let first = iter.next();
            let more_than_one = iter.next().is_some();

            // if val is not already present in column
            if first.is_some() && !more_than_one && !sudoku.row(row_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                grid[first.unwrap()][row_i as usize].iter_mut().for_each(|lit| *lit = false);
                grid[first.unwrap()][row_i as usize][val as usize - 1] = true;

                *sudoku.cell_mut(first.unwrap() as u32, row_i) = Cell::Occupied(val);
                comments.push(format!("{}/{row_i} is trivially {}", first.unwrap(), val));
            }
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let cells_in_block = block_iter(block_x, block_y, sudoku.n);

            for val in 1..=sudoku.n.pow(2) {
                let mut iter = cells_in_block.iter()
                    .filter_map(|(x, y)| if grid[*x as usize][*y as usize][val as usize - 1] { Some((*x, *y)) } else { None });
                let first = iter.next();
                let more_than_one = iter.next().is_some();

                if first.is_some() && !more_than_one && !cells_in_block.iter().any(|(x, y)| matches!(sudoku.cell(*x, *y), Cell::Occupied(num) if *num == val)) {
                    cells_in_block.iter().for_each(|(x, y)| grid[*x as usize][*y as usize][val as usize - 1] = false);

                    let (x, y) = first.unwrap();
                    grid[x as usize][y as usize][val as usize - 1] = true;

                    *sudoku.cell_mut(x, y) = Cell::Occupied(val);
                    comments.push(format!("{x}/{y} is trivially {val}"));
                }
            }
        }
    }

    if comments.is_empty() {
        None
    } else {
        Some(comments)
    }
}

fn build_value_grid_and_optimize(sudoku: &mut Sudoku, solver: &mut Solver<impl SolverImpl>, var_map: &mut HashMap<String, Lit>) -> (Vec<Vec<Vec<Option<Lit>>>>, Vec<String>) {
    let mut grid: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut comments = vec![];

    for _x in 0..sudoku.n.pow(2) {
        grid.push(vec![]);
        for _y in 0..sudoku.n.pow(2) {
            grid.last_mut().unwrap().push(vec![false; sudoku.n.pow(2) as usize]);

            for val in 1..=sudoku.n.pow(2) {
                grid.last_mut().unwrap().last_mut().unwrap()[val as usize - 1] = true;
            }
        }
    }

    loop {
        propagate_occupied_cells(sudoku, &mut grid);

        if let Some(mut new_comments) = make_trivial_choices(sudoku, &mut grid) {
            comments.append(&mut new_comments);
        } else {
            break;
        }
    }

    let mut lit_grid = vec![vec![vec![None; sudoku.n.pow(2) as usize]; sudoku.n.pow(2) as usize]; sudoku.n.pow(2) as usize];

    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            for val in 1..=sudoku.n.pow(2) {
                if grid[x as usize][y as usize][val as usize - 1] {
                    let lit = solver.new_lit();
                    lit_grid[x as usize][y as usize][val as usize - 1] = Some(lit);
                    var_map.insert(format!("{x}/{y} is {val}"), lit);
                }
            }
        }
    }

    (lit_grid, comments)
}

fn block_iter(block_x: u32, block_y: u32, n: u32) -> Vec<(u32, u32)> {
    let x_offset = block_x * n;
    let y_offset = block_y * n;

    let mut result = vec![];
    for x in x_offset..(x_offset + n) {
        for y in y_offset..(y_offset + n) {
            result.push((x, y));
        }
    }
    result
}

fn encode(sudoku: &mut Sudoku, solver: &mut Solver<impl SolverImpl>) -> (HashMap<String, Lit>, Vec<String>) {
        let mut var_map = HashMap::new();
    let (potential_value_grid, trivial_comments) = build_value_grid_and_optimize(sudoku, solver, &mut var_map);

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    for (x, y) in cells.clone() {
        let values = potential_value_grid[x as usize][y as usize].iter().filter_map(|item| *item).collect::<Vec<Lit>>();
        debug_assert_ne!(values.len(), 1);
        if !values.is_empty() {
            solver.at_least_k(AtLeastKStrategy::SequentialCounter, &values, 1);
            // solver.at_least_one(&values);
            // solver.at_most_one(AtMostOneStrategy::Pairwise, &values);
        }
    }

    for col_i in one_axis_coords.clone() {
        for val in vals.clone() {
            let mut tmp = vec![];
            let values = potential_value_grid[col_i as usize].iter().enumerate()
                .filter_map(|(row_i, cell)| {
                    if cell[val as usize - 1].is_some() { tmp.push((col_i, row_i)); }
                    cell[val as usize - 1]
                }).collect::<Vec<Lit>>();
            debug_assert_ne!(values.len(), 1);

            // if val is already present in column
            if !sudoku.col(col_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                // solver.exactly_k(ExactlyKStrategy::SequentialCounter, &values, 1);
                solver.at_least_one(&values);
                solver.at_most_one(AtMostOneStrategy::Pairwise, &values);
            } else {
                debug_assert!(values.is_empty());
            }
        }
    }

    for row_i in one_axis_coords.clone() {
        for val in vals.clone() {
            let values = potential_value_grid.iter()
                .map(move |col| &col[row_i as usize])
                .filter_map(|cell| cell[val as usize - 1]).collect::<Vec<Lit>>();
            debug_assert_ne!(values.len(), 1);

            // if val is not already present in column
            if !sudoku.row(row_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                // solver.exactly_k(ExactlyKStrategy::SequentialCounter, &values, 1);
                solver.at_least_one(&values);
                solver.at_most_one(AtMostOneStrategy::Pairwise, &values);
            } else {
                debug_assert!(values.is_empty());
            }
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let cells_in_block = block_iter(block_x, block_y, sudoku.n);

            for val in vals.clone() {
                let values = cells_in_block.iter()
                    .filter_map(|(x, y)| potential_value_grid[*x as usize][*y as usize][val as usize - 1]).collect::<Vec<Lit>>();
                debug_assert_ne!(values.len(), 1);

                if !cells_in_block.iter().any(|(x, y)| matches!(sudoku.cell(*x, *y), Cell::Occupied(num) if *num == val)) {
                    // solver.exactly_k(ExactlyKStrategy::SequentialCounter, &values, 1);
                    solver.at_least_one(&values);
                    solver.at_most_one(AtMostOneStrategy::Pairwise, &values);
                } else {
                    debug_assert!(values.is_empty());
                }
            }
        }
    }

    (var_map, trivial_comments)
}

pub fn find_solution(sudoku: &Sudoku, timeout: Duration) -> SatProblemResult<Sudoku> {
    let mut sudoku = sudoku.clone();

    let mut solver = Solver::<ipasir::Solver>::new();
    let (var_map, _) = encode(&mut sudoku, &mut solver);

    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    match solver.solve_with_timeout(timeout) {
        SolveWithTimeoutResult::Sat => {
            for (x, y) in cells.clone() {
                for val in vals.clone() {
                    if let Some(id) = var_map.get(&format!("{x}/{y} is {val}")) {
                        if let LitValue::True = solver.val(*id) {
                            *sudoku.cell_mut(x, y) = Cell::Occupied(val);
                        }
                    }
                }
            }

            SatProblemResult::Sat(sudoku)
        }
        SolveWithTimeoutResult::TimeoutReached => SatProblemResult::Timeout,
        SolveWithTimeoutResult::Unsat => SatProblemResult::Unsat,
    }
}

pub fn gen_dimacs(sudoku: &Sudoku) -> String {
    let mut sudoku = sudoku.clone();

    let mut solver = Solver::<dimacs_emitting::Solver>::new();
    let (var_map, comments) = encode(&mut sudoku, &mut solver);
    comments.into_iter().for_each(|c| solver.implementation.add_comment(c));

    for (key, value) in var_map {
        solver.implementation.add_comment(format!("{key} <=> {}", value.id));
    }

    solver.implementation.get_dimacs()
}
