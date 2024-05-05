use std::collections::HashMap;

use crate::ex1::coloring::FindKResult;
use crate::ex1::coloring::graph::Graph;
use crate::solver::{ipasir, Solver, SolverImpl, SolveWithTimeoutResult};
use crate::solver::literal::Lit;
use crate::util::Timer;

fn find_k_inner(graph: &Graph, timer: Timer) -> FindKResult {
    let mut num_bits = 1;

    loop {
        println!("trying {num_bits} bits");
        let mut var_map = HashMap::new();
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

                let a_bit = *var_map.entry(format!("v{a}_b{bit}")).or_insert(solver.new_lit());
                let b_bit = *var_map.entry(format!("v{b}_b{bit}")).or_insert(solver.new_lit());

                let diff = *var_map.entry(format!("v{a}_v{b}_b{bit}_diff")).or_insert(solver.new_lit());

                solver.add_clause([-diff, a_bit, b_bit]);
                solver.add_clause([-diff, -a_bit, -b_bit]);

                solver.add_clause([diff, -a_bit, b_bit]);
                solver.add_clause([diff, a_bit, -b_bit]);
                diff_vars.push(diff);
            }
            solver.add_clause(diff_vars);
        }

        if solver.solve_with_timeout(timer.time_left().unwrap()) == SolveWithTimeoutResult::Sat {
            return if let Some(k) = go_back_until_failure(graph, timer, (var_map, solver, num_bits)) {
                FindKResult::Found(k)
            } else {
                FindKResult::TimeoutReached
            };
        }

        num_bits += 1;
    }
}

pub fn go_back_until_failure(graph: &Graph, timer: Timer, tup: (HashMap<String, Lit>, Solver<impl SolverImpl>, u32)) -> Option<u32> {
    let (mut var_map, mut solver, min_bits) = tup;
    for x in (2_u32.pow(min_bits - 1)..2_u32.pow(min_bits)).rev() {
        if timer.has_finished() {
            return None;
        }
        println!("disabling color {x}");

        for v in 1..=graph.num_vertices {
            let mut solver_w_open_clause = solver.start_clause();
            for b in 0..min_bits {
                let lit = *var_map.entry(format!("v{v}_b{b}")).or_insert(solver_w_open_clause.new_lit());
                assert!(!lit.negated);

                if (x & (1 << b) as u32) == 0 {
                    solver_w_open_clause.add_literal(lit);
                } else {
                    solver_w_open_clause.add_literal(-lit);
                }
            }
            solver = solver_w_open_clause.end_clause();
        }

        let result = solver.solve_with_timeout(timer.time_left().unwrap());
        println!("{result:?}");
        match result {
            SolveWithTimeoutResult::Unsat => return Some(x + 1),
            SolveWithTimeoutResult::TimeoutReached => return None,
            SolveWithTimeoutResult::Sat => {}
        }
    }

    if min_bits == 1 {
        return Some(1);
    }
    unreachable!()
}

#[allow(dead_code)]
pub fn find_k(graph: Graph, timer: Timer) -> FindKResult {
    if let FindKResult::Found(k) = find_k_inner(&graph, timer) {
        FindKResult::Found(k)
    } else {
        FindKResult::TimeoutReached
    }
}