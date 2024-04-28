use std::collections::HashMap;
use std::ffi::c_uint;
use std::ops::RangeFrom;

use crate::cnf::literal::Lit;
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

                solver.add_clause(&[-Lit::new(diff_id1), Lit::new(a_bit_id).into(), Lit::new(b_bit_id).into()]);
                solver.add_clause(&[-Lit::new(diff_id1), -Lit::new(a_bit_id), -Lit::new(b_bit_id)]);
                diff_vars.push(Lit::new(diff_id1));
            }
            solver.add_clause(&diff_vars);
        }

        if solver.solve() == SolveResult::Sat {
            if let Some(k) = go_back_until_failure(graph, timer, (var_map, solver, num_bits, allocator)) {
                return FindKResult::Found(k);
            } else {
                return FindKResult::TimeoutReached;
            }
        }
        num_bits += 1;
    }
}

pub fn go_back_until_failure(graph: &Graph, timer: Timer, tup: (HashMap<String, c_uint>, Solver, u32, RangeFrom<c_uint>)) -> Option<u32> {
    let (mut var_map, mut solver, min_bits, mut allocator) = tup;
    for x in (2_u32.pow(min_bits - 1)..2_u32.pow(min_bits)).rev() {
        if timer.has_finished() {
            return None;
        }
        println!("disabling color {x}");

        for v in 1..=graph.num_vertices {
            for b in 0..min_bits {
                let id = *var_map.entry(format!("v{}_b{}", v, b)).or_insert(allocator.next().unwrap());
                
                if  (x & (1 << b) as u32) == 0 {
                    solver.add_literal(-Lit::new(id));
                } else {
                    solver.add_literal(Lit::new(id));
                }
            }
            solver.add_literal(Lit::clause_end());
        }

        let result = solver.solve();
        if result == SolveResult::Unsat {
            return Some(x + 1);
        }
    }

    if min_bits == 1 {
        return Some(1);
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