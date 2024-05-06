use crate::literal::Lit;
use crate::{AtMostKStrategy, Solver, SolverImpl};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AtMostOneStrategy {
    SequentialCounter,
    Pairwise,
}

impl AtMostOneStrategy {
    pub fn encode(self,  lits: &[Lit], solver: &mut Solver<impl SolverImpl>) {
        match self {
            AtMostOneStrategy::SequentialCounter => {
                AtMostKStrategy::SequentialCounter.encode(lits, 1, solver)
            }
            AtMostOneStrategy::Pairwise => {
                for i in 0..lits.len() {
                    for j in (i + 1)..lits.len() {
                        solver.add_clause([-lits[i], -lits[j]]);
                    }
                }
            }
        }
    }
}
