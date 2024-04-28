use std::collections::HashMap;

use crate::cnf::literal::{Literal, Variable};
use crate::ex1::coloring::FindKResult;
use crate::ex1::coloring::graph::Graph;
use crate::solver::{Solver, SolveResult};
use crate::util::Timer;

fn find_min_no_bits(graph: &Graph, timer: Timer) -> FindKResult {
    let mut allocator = 1..;
    let mut var_map = HashMap::new();

    let mut num_bits = 1;

    loop {
        println!("trying {num_bits} bits");
        let mut solver = Solver::new();

        if timer.has_finished() {
            return FindKResult::TimeoutReached;
        } else {
            solver.set_terminate(move || {
                timer.has_finished()
            });
        }

        for edge in &graph.edges {
            let mut diff_vars = vec![];
            for bit in 0..num_bits {
                let mut a = edge.0;
                let mut b = edge.1;
                if a < b {
                    std::mem::swap(&mut a, &mut b);
                }

                let a_bit_id = *var_map.entry(format!("v{}_b{}", a, bit)).or_insert(allocator.next().unwrap());
                let b_bit_id = *var_map.entry(format!("v{}_b{}", b, bit)).or_insert(allocator.next().unwrap());

                let diff_id1 = *var_map.entry(format!("v{}_v{}_b{}_diff1", a, b, bit)).or_insert(allocator.next().unwrap());

                solver.add_clause(&[-Variable::new(diff_id1), Variable::new(a_bit_id).into(), Variable::new(b_bit_id).into()]);
                solver.add_clause(&[-Variable::new(diff_id1), -Variable::new(a_bit_id), -Variable::new(b_bit_id)]);
                diff_vars.push(Literal::new(Variable::new(diff_id1), false));
            }
            solver.add_clause(&diff_vars);
        }

        if solver.solve() == SolveResult::Sat {
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
    let mut solver = Solver::new();
    let mut var_map = HashMap::new();
    let mut allocator = 1..;

    solver.set_terminate(move || timer.has_finished());

    for vertex in 1..=graph.num_vertices {
        for color in 0..2_u32.pow(min_bits) {
            let v_is_color = *var_map.entry(format!("v{}_c{}", vertex, color)).or_insert(allocator.next().unwrap());
            solver.add_literal(Literal::new_pos(v_is_color));
        }
        solver.add_literal(Literal::clause_end());
    }

    for edge in &graph.edges {
        for color in 0..2_u32.pow(min_bits) {
            let a_is_color = *var_map.entry(format!("v{}_c{}", edge.0, color)).or_insert(allocator.next().unwrap());
            let b_is_color = *var_map.entry(format!("v{}_c{}", edge.1, color)).or_insert(allocator.next().unwrap());
            solver.add_clause(&[Literal::new_neg(a_is_color), Literal::new_neg(b_is_color)]);
        }
    }

    for color in 0..2_u32.pow(min_bits) {
        for vertex in 1..=graph.num_vertices {
            let v_is_color = *var_map.entry(format!("v{}_c{}", vertex, color)).or_insert(allocator.next().unwrap());
            solver.add_clause(&[Literal::new_neg(v_is_color)]);
        }

        if timer.has_finished() {
            return None;
        }
        match solver.solve() {
            SolveResult::Sat => {}
            SolveResult::Unsat => return Some(2_u32.pow(min_bits) - color),
            SolveResult::Interrupted => return None,
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

