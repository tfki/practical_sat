use crate::solver::literal::Lit;
use crate::solver::{Solver, SolverImpl};

mod at_most_one;
pub use at_most_one::AtMostOneStrategy;

mod at_most_k;
pub use at_most_k::AtMostKStrategy;

mod at_least_k;
pub use at_least_k::AtLeastKStrategy;

mod exactly_k;
pub use exactly_k::ExactlyKStrategy;

mod exactly_one;
pub use exactly_one::ExactlyOneStrategy;

fn seq_counter(lits: &[Lit], solver: &mut Solver<impl SolverImpl>) -> Vec<Lit> {
    let mut prev_layer_outputs = vec![lits[0]];

    for lit in &lits[1..] {
        let mut layer_outputs = vec![];
        for _ in 0..=prev_layer_outputs.len() {
            layer_outputs.push(solver.new_lit());
        };

        solver.add_clause([layer_outputs[0], -*lit]);
        solver.add_clause([layer_outputs[0], -prev_layer_outputs[0]]);
        solver.add_clause([-layer_outputs[0], prev_layer_outputs[0], *lit]);

        for (i, layer_output) in layer_outputs.iter().enumerate() {
            if i == 0 || i == layer_outputs.len() - 1 { continue; }

            solver.add_clause([-*layer_output, prev_layer_outputs[i], *lit]);
            solver.add_clause([-*layer_output, prev_layer_outputs[i], prev_layer_outputs[i - 1]]);

            solver.add_clause([*layer_output, -prev_layer_outputs[i], -prev_layer_outputs[i - 1]]);
            solver.add_clause([*layer_output, -prev_layer_outputs[i], -*lit]);
            solver.add_clause([*layer_output, -prev_layer_outputs[i - 1], -*lit]);
        }

        solver.add_clause([-*layer_outputs.last().unwrap(), *lit]);
        solver.add_clause([-*layer_outputs.last().unwrap(), *prev_layer_outputs.last().unwrap()]);

        solver.add_clause([*layer_outputs.last().unwrap(), -*lit, -*prev_layer_outputs.last().unwrap()]);
        prev_layer_outputs = layer_outputs;
    }

    prev_layer_outputs
}
