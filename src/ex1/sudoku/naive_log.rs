use std::collections::HashMap;
use std::time::Duration;

use crate::ex1::sudoku::{Cell, Sudoku};
use crate::SatProblemResult;
use crate::SatProblemResult::Sat;
use crate::solver::{ipasir, LitValue, Solver, SolverImpl, SolveWithTimeoutResult};
use crate::solver::literal::Lit;

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

fn propagate_occupied_cells(sudoku: &Sudoku, grid: &mut Vec<Vec<Vec<Option<Lit>>>>) {
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
}

fn make_trivial_choices(sudoku: &mut Sudoku, grid: &mut Vec<Vec<Vec<Option<Lit>>>>) -> Option<Vec<String>> {
    let mut comments = vec![];

    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            if grid[x as usize][y as usize].iter().filter(|item| item.is_some()).count() == 1 {
                let (index, _) = grid[x as usize][y as usize].iter().enumerate().find(|(_, item)| item.is_some()).unwrap();
                *sudoku.cell_mut(x, y) = Cell::Occupied(index as u32 + 1);
                comments.push(format!("{x}/{y} is trivially {}", index + 1));
            }
        }
    }

    for col_i in 0..sudoku.n.pow(2) {
        for val in 1..=sudoku.n.pow(2) {
            let mut iter = grid[col_i as usize].iter().enumerate()
                .filter_map(|(row_i, col)| col[val as usize - 1].map(|_| row_i));
            let first = iter.next();
            let more_than_one = iter.next().is_some();

            if first.is_some() && !more_than_one && !sudoku.col(col_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                grid[col_i as usize][first.unwrap()].iter_mut().for_each(|lit| *lit = None);
                grid[col_i as usize][first.unwrap()][val as usize - 1] = Some(Lit::new(1));

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
                .filter_map(|(col_i, cell)| cell[val as usize - 1].map(|_| col_i));
            let first = iter.next();
            let more_than_one = iter.next().is_some();

            // if val is not already present in column
            if first.is_some() && !more_than_one && !sudoku.row(row_i).any(|item| matches!(item, Cell::Occupied(num) if *num == val)) {
                grid[first.unwrap()][row_i as usize].iter_mut().for_each(|lit| *lit = None);
                grid[first.unwrap()][row_i as usize][val as usize - 1] = Some(Lit::new(1));

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
                    .filter_map(|(x, y)| grid[*x as usize][*y as usize][val as usize - 1].map(|_| (*x, *y)));
                let first = iter.next();
                let more_than_one = iter.next().is_some();

                if first.is_some() && !more_than_one && !cells_in_block.iter().any(|(x, y)| matches!(sudoku.cell(*x, *y), Cell::Occupied(num) if *num == val)) {
                    cells_in_block.iter().for_each(|(x, y)| grid[*x as usize][*y as usize][val as usize - 1] = None);

                    let (x, y) = first.unwrap();
                    grid[x as usize][y as usize][val as usize - 1] = Some(Lit::new(1));

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
        propagate_occupied_cells(sudoku, &mut grid);

        if let Some(mut new_comments) = make_trivial_choices(sudoku, &mut grid) {
            comments.append(&mut new_comments);
        } else {
            break;
        }
    }

    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            for val in 1..=sudoku.n.pow(2) {
                if let Some(_dummy_lit) = grid[x as usize][y as usize][val as usize - 1] {
                    let lit = solver.new_lit();
                    grid[x as usize][y as usize][val as usize - 1] = Some(lit);
                    var_map.insert(format!("{x}/{y} is {val}"), lit);
                }
            }
        }
    }

    (grid, comments)
}

fn encode(sudoku: &Sudoku) -> (Solver<impl SolverImpl>, HashMap<String, Lit>) {
    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::<String, Lit>::new();
    let mut sudoku = sudoku.clone();

    let bits = (sudoku.n.pow(2) as f32).log2().ceil() as u32;

    let (possible_value_grid, new_comments) = build_value_grid_and_optimize(&mut sudoku, &mut solver, &mut var_map);

    for x in 0..sudoku.n.pow(2) {
        for y in 0..sudoku.n.pow(2) {
            if let Cell::Occupied(num) = sudoku.cell(x, y) {
                for bit in 0..bits {
                    let lit = *var_map.entry(format!("{x}/{y} b{bit}")).or_insert(solver.new_lit());
                    if ((num - 1) & (1 << bit) as u32) == 0 {
                        solver.add_clause([-lit]);
                    } else {
                        solver.add_clause([lit]);
                    }
                }
            } else {
                for val in 0..sudoku.n.pow(2) {
                    if possible_value_grid[x as usize][y as usize][val as usize].is_none() {
                        let mut solver_w_open_clause = solver.start_clause();
                        for bit in 0..bits {
                            let lit = *var_map.entry(format!("{x}/{y} b{bit}")).or_insert(solver_w_open_clause.new_lit());
                            if (val & (1 << bit) as u32) == 0 {
                                solver_w_open_clause.add_literal(lit);
                            } else {
                                solver_w_open_clause.add_literal(-lit);
                            }
                        }
                        solver = solver_w_open_clause.end_clause();
                    }
                }
                for val in sudoku.n.pow(2)..2_u32.pow(bits) {                    
                    let mut solver_w_open_clause = solver.start_clause();
                    for bit in 0..bits {
                        let lit = *var_map.entry(format!("{x}/{y} b{bit}")).or_insert(solver_w_open_clause.new_lit());
                        if (val & (1 << bit) as u32) == 0 {
                            solver_w_open_clause.add_literal(lit);
                        } else {
                            solver_w_open_clause.add_literal(-lit);
                        }
                    }
                    solver = solver_w_open_clause.end_clause();
                }
            }
        }
    }

    for col_k in 0..sudoku.n.pow(2) {
        for i in 0..sudoku.n.pow(2) {
            for j in (i + 1)..sudoku.n.pow(2) {
                let mut diff_bits = vec![];

                for bit in 0..bits {
                    let a = *var_map.entry(format!("{col_k}/{i} b{bit}")).or_insert(solver.new_lit());
                    let b = *var_map.entry(format!("{col_k}/{j} b{bit}")).or_insert(solver.new_lit());
                    let diff = *var_map.entry(format!("{col_k}/{i}, {col_k}/{j} differ in b{bit}")).or_insert(solver.new_lit());

                    solver.add_clause([-diff, a, b]);
                    solver.add_clause([-diff, -a, -b]);

                    solver.add_clause([diff, -a, b]);
                    solver.add_clause([diff, a, -b]);

                    diff_bits.push(diff);
                }

                solver.add_clause(diff_bits);
            }
        }
    }

    for row_k in 0..sudoku.n.pow(2) {
        for i in 0..sudoku.n.pow(2) {
            for j in (i + 1)..sudoku.n.pow(2) {
                let mut diff_bits = vec![];

                for bit in 0..bits {
                    let a = *var_map.entry(format!("{i}/{row_k} b{bit}")).or_insert(solver.new_lit());
                    let b = *var_map.entry(format!("{j}/{row_k} b{bit}")).or_insert(solver.new_lit());
                    let diff = *var_map.entry(format!("{i}/{row_k}, {j}/{row_k} differ in b{bit}")).or_insert(solver.new_lit());

                    solver.add_clause([-diff, a, b]);
                    solver.add_clause([-diff, -a, -b]);

                    solver.add_clause([diff, -a, b]);
                    solver.add_clause([diff, a, -b]);

                    diff_bits.push(diff);
                }

                solver.add_clause(diff_bits);
            }
        }
    }

    for block_x in 0..sudoku.n {
        for block_y in 0..sudoku.n {
            let block = block_iter(block_x, block_y, sudoku.n);

            for i in 0..sudoku.n.pow(2) {
                for j in (i + 1)..sudoku.n.pow(2) {
                    let a = block[i as usize];
                    let b = block[j as usize];

                    let mut diff_bits = vec![];

                    for bit in 0..bits {
                        let a_lit = *var_map.entry(format!("{}/{} b{bit}", a.0, a.1)).or_insert(solver.new_lit());
                        let b_lit = *var_map.entry(format!("{}/{} b{bit}", b.0, b.1)).or_insert(solver.new_lit());
                        let diff = *var_map.entry(format!("{}/{}, {}/{} differ in b{bit}", a.0, a.1, b.0, b.1)).or_insert(solver.new_lit());

                        solver.add_clause([-diff, a_lit, b_lit]);
                        solver.add_clause([-diff, -a_lit, -b_lit]);

                        solver.add_clause([diff, -a_lit, b_lit]);
                        solver.add_clause([diff, a_lit, -b_lit]);

                        diff_bits.push(diff);
                    }

                    solver.add_clause(diff_bits);
                }
            }
        }
    }

    (solver, var_map)
}

pub fn find_solution(sudoku: &Sudoku, timeout: Duration) -> SatProblemResult<Sudoku> {
    let (mut solver, var_map) = encode(sudoku);
    let mut solution = sudoku.clone();

    let bits = (sudoku.n.pow(2) as f32).log2().ceil() as u32;
    match solver.solve_with_timeout(timeout) {
        SolveWithTimeoutResult::Sat => {
            let mut keys = var_map.keys().cloned().collect::<Vec<String>>();
            keys.sort();

            for x in 0..sudoku.n.pow(2) {
                for y in 0..sudoku.n.pow(2) {
                    let mut num = 0;

                    for bit in 0..bits {
                        let lit = var_map[&format!("{x}/{y} b{bit}")];
                        if solver.val(lit) == LitValue::True {
                            num += 1 << bit;
                        }
                    }

                    num += 1;
                    *solution.cell_mut(x, y) = Cell::Occupied(num);
                }
            }

            Sat(solution)
        }
        SolveWithTimeoutResult::Unsat => SatProblemResult::Unsat,
        SolveWithTimeoutResult::TimeoutReached => SatProblemResult::Timeout,
    }
}

pub fn gen_dimacs(sudoku: &Sudoku) -> String {
    todo!()
}
