use std::collections::HashMap;
use std::ops::RangeFrom;

use crate::{dimacs_emitting_solver, SatProblemResult};
use crate::cnf::literal::Lit;
use crate::ex1::sudoku::{Cell, Sudoku};
use crate::solver::{LitValue, Solver, SolveResult};
use crate::util::Timer;

fn build_value_grid(sudoku: &mut Sudoku, allocator: &mut RangeFrom<u32>, var_map: &mut HashMap<String, Lit>, no_opt_trivial: bool) -> (Vec<Vec<Vec<Option<Lit>>>>, Vec<String>) {
    let mut grid: Vec<Vec<Vec<Option<Lit>>>> = Vec::new();
    let mut comments = vec![];

    for _x in 0..sudoku.n.pow(2) {
        grid.push(vec![]);
        for _y in 0..sudoku.n.pow(2) {
            grid.last_mut().unwrap().push(vec![None; sudoku.n.pow(2) as usize]);

            for val in 1..=sudoku.n.pow(2) {
                // 1 is just a dummy, will get replaced by actual variable later
                let lit = Lit::new(1);
                grid.last_mut().unwrap().last_mut().unwrap()[val as usize - 1] = Some(lit);
            }
        }
    }

    loop {
        for x in 0..sudoku.n.pow(2) {
            for y in 0..sudoku.n.pow(2) {
                if let Cell::Occupied(num) = sudoku.cell(x, y) {
                    grid[x as usize][y as usize].iter_mut().for_each(|cell| *cell = None);

                    let block_x = x / sudoku.n;
                    let block_y = y / sudoku.n;

                    let x_offset = block_x * sudoku.n;
                    let y_offset = block_y * sudoku.n;

                    for x in x_offset..(x_offset + sudoku.n) {
                        for y in y_offset..(y_offset + sudoku.n) {
                            grid[x as usize][y as usize][*num as usize - 1] = None;
                        }
                    }

                    for x in 0..sudoku.n.pow(2) {
                        grid[x as usize][y as usize][*num as usize - 1] = None;
                    }
                    for y in 0..sudoku.n.pow(2) {
                        grid[x as usize][y as usize][*num as usize - 1] = None;
                    }
                }
            }
        }
        if no_opt_trivial { break; }

        let mut all_done = true;
        for x in 0..sudoku.n.pow(2) {
            for y in 0..sudoku.n.pow(2) {
                if grid[x as usize][y as usize].iter().filter(|item| item.is_some()).count() == 1 {
                    let (index, _) = grid[x as usize][y as usize].iter().enumerate().find(|(_, item)| item.is_some()).unwrap();
                    *sudoku.cell_mut(x, y) = Cell::Occupied(index as u32 + 1);
                    comments.push(format!("{x}/{y} is trivially {}", index + 1));
                    all_done = false;
                }
            }
        }
        if all_done { break; }
    }

    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            for val in 1..=sudoku.n.pow(2) {
                if let Some(_dummy_lit) = grid[x as usize][y as usize][val as usize - 1] {
                    let lit = Lit::new(allocator.next().unwrap());
                    grid[x as usize][y as usize][val as usize - 1] = Some(lit);
                    var_map.insert(format!("{x}/{y} is {val}"), lit);
                }
            }
        }
    }

    (grid, comments)
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

pub fn find_solution(sudoku: &Sudoku, timer: Timer) -> SatProblemResult<Sudoku> {
    let mut sudoku = sudoku.clone();

    let mut solver = Solver::new();
    let mut var_map = HashMap::new();
    let mut allocator = 1..;
    let (potential_value_grid, _) = build_value_grid(&mut sudoku, &mut allocator, &mut var_map, false);

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    for (x, y) in cells.clone() {
        let values = potential_value_grid[x as usize][y as usize].iter().filter_map(|item| *item).collect::<Vec<Lit>>();
        if !values.is_empty() {
            solver.at_least_one(&values);
            solver.at_most_one_pairwise(&values);
        }
    }

    for col_i in one_axis_coords.clone() {
        for val in vals.clone() {
            let values = potential_value_grid[col_i as usize].iter()
                .filter_map(|col| col[val as usize - 1]).collect::<Vec<Lit>>();

            // if val is already present in column
            if !sudoku.col(col_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                solver.at_least_one(&values);
                solver.at_most_one_pairwise(&values);
            }
        }
    }

    for row_i in one_axis_coords.clone() {
        for val in vals.clone() {
            let values = potential_value_grid.iter()
                .map(move |col| &col[row_i as usize])
                .filter_map(|cell| cell[val as usize - 1]).collect::<Vec<Lit>>();

            // if val is not already present in column
            if !sudoku.row(row_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                solver.at_least_one(&values);
                solver.at_most_one_pairwise(&values);
            }
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let cells_in_block = block_iter(block_x, block_y, sudoku.n);

            for val in vals.clone() {
                let values = cells_in_block.iter()
                    .filter_map(|(x, y)| potential_value_grid[*x as usize][*y as usize][val as usize - 1]).collect::<Vec<Lit>>();

                if !cells_in_block.iter().any(|(x, y)| matches!(sudoku.cell(*x, *y), Cell::Occupied(num) if *num == val)) {
                    solver.at_least_one(&values);
                    solver.at_most_one_pairwise(&values);
                }
            }
        }
    }

    solver.set_terminate(move || timer.has_finished());
    match solver.solve() {
        SolveResult::Sat => {
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
        SolveResult::Interrupted => SatProblemResult::Timeout,
        SolveResult::Unsat => SatProblemResult::Unsat,
    }
}

pub fn gen_dimacs(sudoku: &Sudoku, no_opt_trivial: bool) -> String {
    let mut sudoku = sudoku.clone();

    let mut solver = dimacs_emitting_solver::Solver::new();
    let mut var_map = HashMap::new();
    let mut allocator = 1..;
    let (potential_value_grid, trivial_assignment_comments) = build_value_grid(&mut sudoku, &mut allocator, &mut var_map, no_opt_trivial);
    trivial_assignment_comments.into_iter().for_each(|string| solver.add_comment(string));

    // convenience iterators
    let vals = 1..=sudoku.n.pow(2);
    let one_axis_coords = 0..sudoku.n.pow(2);
    let cells = one_axis_coords.clone().flat_map(|x| one_axis_coords.clone().map(move |y| (x, y)));

    for (x, y) in cells.clone() {
        let values = potential_value_grid[x as usize][y as usize].iter().filter_map(|item| *item).collect::<Vec<Lit>>();
        if !values.is_empty() {
            solver.at_least_one(&values);
            solver.at_most_one_pairwise(&values);
        }
    }

    for col_i in one_axis_coords.clone() {
        for val in vals.clone() {
            let values = potential_value_grid[col_i as usize].iter()
                .filter_map(|col| col[val as usize - 1]).collect::<Vec<Lit>>();

            // if val is already present in column
            if !sudoku.col(col_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                solver.at_least_one(&values);
                solver.at_most_one_pairwise(&values);
            }
        }
    }

    for row_i in one_axis_coords.clone() {
        for val in vals.clone() {
            let values = potential_value_grid.iter()
                .map(move |col| &col[row_i as usize])
                .filter_map(|cell| cell[val as usize - 1]).collect::<Vec<Lit>>();

            // if val is not already present in column
            if !sudoku.row(row_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                solver.at_least_one(&values);
                solver.at_most_one_pairwise(&values);
            }
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let cells_in_block = block_iter(block_x, block_y, sudoku.n);

            for val in vals.clone() {
                let values = cells_in_block.iter()
                    .filter_map(|(x, y)| potential_value_grid[*x as usize][*y as usize][val as usize - 1]).collect::<Vec<Lit>>();

                if !cells_in_block.iter().any(|(x, y)| matches!(sudoku.cell(*x, *y), Cell::Occupied(num) if *num == val)) {
                    solver.at_least_one(&values);
                    solver.at_most_one_pairwise(&values);
                }
            }
        }
    }

    for (key, value) in var_map {
        solver.add_comment(format!("{key} <=> {}", value.id));
    }

    solver.get_dimacs()
}
