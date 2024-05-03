use crate::solver::literal::Lit;
use crate::solver::{Solver, SolverImpl};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ExactlyOneStrategy {
    SequentialCounter,
    OrAndPairwise,
}

impl ExactlyOneStrategy {
    pub fn encode(self, _lits: &[Lit], _solver: &mut Solver<impl SolverImpl>) {
        todo!()
    }
}
