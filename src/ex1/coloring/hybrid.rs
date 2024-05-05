use std::collections::HashMap;

use crate::ex1::coloring::FindKResult;
use crate::ex1::coloring::graph::Graph;
use crate::solver::{ipasir, Solver, SolveWithTimeoutResult};
use crate::util::Timer;

fn find_min_no_bits(graph: &Graph, timer: Timer) -> FindKResult {
    let mut var_map = HashMap::new();

    let mut num_bits = 1;

    loop {
        println!("trying {num_bits} bits");
        let mut solver = Solver::<ipasir::Solver>::new();

        if timer.has_finished() {
            return FindKResult::TimeoutReached;
        }

        for edge in &graph.edges {
            let mut diff_vars = vec![];
            for bit in 0..num_bits {
                let mut a = edge.0;
                let mut b = edge.1;
                if a < b {
                    std::mem::swap(&mut a, &mut b);
                }

                let a_bit = *var_map.entry(format!("v{}_b{}", a, bit)).or_insert(solver.new_lit());
                let b_bit = *var_map.entry(format!("v{}_b{}", b, bit)).or_insert(solver.new_lit());

                let diff = *var_map.entry(format!("v{}_v{}_b{}_diff1", a, b, bit)).or_insert(solver.new_lit());

                solver.add_clause([-diff, a_bit, b_bit]);
                solver.add_clause([-diff, -a_bit, -b_bit]);
                diff_vars.push(diff);
            }
            solver.add_clause(diff_vars);
        }

        if solver.solve_with_timeout(timer.time_left().unwrap()) == SolveWithTimeoutResult::Sat {
            if let Some(k) = go_back_until_failure_one_hot(graph, timer, num_bits) {
                return FindKResult::Found(k);
            } else {
                return FindKResult::TimeoutReached;
            }
        }
        num_bits += 1;
    }
}

pub fn go_back_until_failure_one_hot(graph: &Graph, timer: Timer, min_bits: u32) -> Option<u32> {
    let mut solver = Solver::<ipasir::Solver>::new();
    let mut var_map = HashMap::new();

    for vertex in 1..=graph.num_vertices {
        let mut solver_w_open_clause = solver.start_clause();
        for color in 0..2_u32.pow(min_bits) {
            let v_is_color = *var_map.entry(format!("v{}_c{}", vertex, color)).or_insert(solver_w_open_clause.new_lit());
            solver_w_open_clause.add_literal(v_is_color);
        }
        solver = solver_w_open_clause.end_clause();
    }

    for edge in &graph.edges {
        for color in 0..2_u32.pow(min_bits) {
            let a_is_color = *var_map.entry(format!("v{}_c{}", edge.0, color)).or_insert(solver.new_lit());
            let b_is_color = *var_map.entry(format!("v{}_c{}", edge.1, color)).or_insert(solver.new_lit());
            solver.add_clause([-a_is_color, -b_is_color]);
        }
    }

    for color in 0..2_u32.pow(min_bits) {
        for vertex in 1..=graph.num_vertices {
            let v_is_color = *var_map.entry(format!("v{}_c{}", vertex, color)).or_insert(solver.new_lit());
            solver.add_clause([-v_is_color]);
        }

        if timer.has_finished() {
            return None;
        }
        match solver.solve_with_timeout(timer.time_left().unwrap()) {
            SolveWithTimeoutResult::Sat => {}
            SolveWithTimeoutResult::Unsat => return Some(2_u32.pow(min_bits) - color),
            SolveWithTimeoutResult::TimeoutReached => return None,
        }
    }

    unreachable!()
}

#[allow(dead_code)]
pub fn find_k(graph: Graph, timer: Timer) -> FindKResult {
    if let FindKResult::Found(k) = find_min_no_bits(&graph, timer) {
        FindKResult::Found(k)
    } else {
        FindKResult::TimeoutReached
    }
}

