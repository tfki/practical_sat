use crate::{Solver, SolverImpl};
use crate::constraints::seq_counter;
use crate::literal::Lit;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ExactlyKStrategy {
    SequentialCounter,
}

impl ExactlyKStrategy {
    pub fn encode(self, lits: &[Lit], k: u32, solver: &mut Solver<impl SolverImpl>) {
        match self {
            ExactlyKStrategy::SequentialCounter => {
                if k > lits.len() as u32 {
                    solver.add_clause([]);
                    return;
                }

                let last_layer_outputs = seq_counter(lits, solver, k);

                if k > 0 {
                    if let Some(x) = last_layer_outputs.get((k - 1) as usize) {
                        solver.add_clause([*x]);
                    }
                }
                if let Some(x) = last_layer_outputs.get(k as usize) {
                    solver.add_clause([-*x]);
                }
            }
        }
    }
}
