pub use at_least_k::AtLeastKStrategy;
pub use at_most_k::AtMostKStrategy;
pub use at_most_one::AtMostOneStrategy;
pub use exactly_k::ExactlyKStrategy;
pub use exactly_one::ExactlyOneStrategy;

use crate::solver::{Solver, SolverImpl};
use crate::solver::literal::Lit;

mod at_most_one;

mod at_most_k;

mod at_least_k;

mod exactly_k;

mod exactly_one;

fn seq_counter(lits: &[Lit], solver: &mut Solver<impl SolverImpl>, max_k: u32) -> Vec<Lit> {
    let mut prev_layer_outputs = vec![lits[0]];

    let mut k = 1;
    for lit in &lits[1..] {
        k += 1;

        let mut layer_outputs = vec![];
        for _ in 0..=k.min(max_k as usize) {
            layer_outputs.push(solver.new_lit());
        };

        solver.add_clause([layer_outputs[0], -*lit]);
        solver.add_clause([layer_outputs[0], -prev_layer_outputs[0]]);
        solver.add_clause([-layer_outputs[0], prev_layer_outputs[0], *lit]);

        for (i, layer_output) in layer_outputs.iter().enumerate() {
            if i == 0 { continue; }
            if i == k - 1 { break; }

            solver.add_clause([-*layer_output, prev_layer_outputs[i], *lit]);
            solver.add_clause([-*layer_output, prev_layer_outputs[i], prev_layer_outputs[i - 1]]);

            solver.add_clause([*layer_output, -prev_layer_outputs[i], -prev_layer_outputs[i - 1]]);
            solver.add_clause([*layer_output, -prev_layer_outputs[i], -*lit]);
            solver.add_clause([*layer_output, -prev_layer_outputs[i - 1], -*lit]);
        }

        if layer_outputs.len() > prev_layer_outputs.len() {
            solver.add_clause([-*layer_outputs.last().unwrap(), *lit]);
            solver.add_clause([-*layer_outputs.last().unwrap(), *prev_layer_outputs.last().unwrap()]);

            solver.add_clause([*layer_outputs.last().unwrap(), -*lit, -*prev_layer_outputs.last().unwrap()]);
        }
        prev_layer_outputs = layer_outputs;
    }

    prev_layer_outputs
}
