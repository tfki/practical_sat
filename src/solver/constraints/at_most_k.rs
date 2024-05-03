use crate::solver::{Solver, SolverImpl};
use crate::solver::constraints::seq_counter;
use crate::solver::literal::Lit;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AtMostKStrategy {
    SequentialCounter,
}

impl AtMostKStrategy {
    pub fn encode<T: SolverImpl>(self, lits: &[Lit], k: u32, solver: &mut Solver<T>) {
        match self {
            AtMostKStrategy::SequentialCounter => {
                if k > lits.len() as u32 {
                    solver.add_clause([]);
                }

                let last_layer_outputs = seq_counter(lits, solver);

                if let Some(x) = last_layer_outputs.get(k as usize) {
                    solver.add_clause([-*x]);
                }
            }
        }
    }
}
