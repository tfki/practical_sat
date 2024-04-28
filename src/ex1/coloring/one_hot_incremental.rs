use std::collections::HashMap;

use crate::cnf::literal::Lit;
use crate::ex1::coloring::FindKResult;
use crate::ex1::coloring::graph::Graph;
use crate::solver::{Solver, SolveResult};
use crate::util::Timer;

#[allow(dead_code)]
pub fn find_k(graph: Graph, timer: Timer) -> FindKResult {
    let mut allocator = 1..;
    let mut solver = Solver::new();
    let mut var_map = HashMap::new();
    let mut num_colors = 1;

    solver.set_terminate(move || timer.has_finished());

    let mut deactivate_color_disjunctions = Lit::new(allocator.next().unwrap());
    solver.assume(-deactivate_color_disjunctions);

    for vertex in 1..=graph.num_vertices {
        let v_is_color0 = *var_map.entry(format!("v{}_is_c{}", vertex, 0)).or_insert(allocator.next().unwrap());
        solver.add_clause(&[deactivate_color_disjunctions, Lit::new(v_is_color0)]);
    }

    for edge in &graph.edges {
        for color in 0..num_colors {
            let a = *var_map.entry(format!("v{}_is_c{}", edge.0, color)).or_insert(allocator.next().unwrap());
            let b = *var_map.entry(format!("v{}_is_c{}", edge.1, color)).or_insert(allocator.next().unwrap());

            solver.add_clause(&[-Lit::new(a), -Lit::new(b)]);
        }
    }

    loop {
        if timer.has_finished() {
            return FindKResult::TimeoutReached;
        }
        match solver.solve() {
            SolveResult::Sat => return FindKResult::Found(num_colors),
            SolveResult::Interrupted => return FindKResult::TimeoutReached,
            SolveResult::Unsat => {}
        }
        num_colors += 1;

        println!("trying {num_colors} colors");

        // deactivate all "at least one color" clauses from last iteration
        solver.add_clause(&[deactivate_color_disjunctions]);
        deactivate_color_disjunctions = Lit::new(allocator.next().unwrap());
        solver.assume(-deactivate_color_disjunctions);

        for edge in &graph.edges {
            let a = *var_map.entry(format!("v{}_is_c{}", edge.0, num_colors - 1)).or_insert(allocator.next().unwrap());
            let b = *var_map.entry(format!("v{}_is_c{}", edge.1, num_colors - 1)).or_insert(allocator.next().unwrap());

            solver.add_clause(&[-Lit::new(a), -Lit::new(b)]);
        }

        for vertex in 1..=graph.num_vertices {
            for color in 0..num_colors {
                let v_is_color = *var_map.entry(format!("v{}_is_c{}", vertex, color)).or_insert(allocator.next().unwrap());
                solver.add_literal(Lit::new(v_is_color));
            }
            solver.add_literal(deactivate_color_disjunctions);
            solver.add_literal(Lit::clause_end());
        }
    }
}

