use crate::solver::literal::Lit;
use crate::solver::{Solver, SolverImpl};
use crate::solver::constraints::seq_counter;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AtLeastKStrategy {
    SequentialCounter,
}

impl AtLeastKStrategy {
    pub fn encode<T: SolverImpl>(self, lits: &[Lit], k: u32, solver: &mut Solver<T>) {
        match self {
            AtLeastKStrategy::SequentialCounter => {
                if k > lits.len() as u32 {
                    solver.add_clause([]);
                }

                let last_layer_outputs = seq_counter(lits, solver, k - 1);

                if k > 0 {
                    if let Some(x) = last_layer_outputs.get((k - 1) as usize) {
                        solver.add_clause([*x]);
                    }
                }
            }
        }
    }
}
