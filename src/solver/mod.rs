use std::ops::RangeFrom;
use std::time::Duration;

use literal::Lit;

pub mod dimacs_emitting;
pub mod ipasir;
mod constraints;
pub mod literal;

pub use constraints::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SolveResult {
    Sat,
    Unsat,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SolveWithTimeoutResult {
    Sat,
    Unsat,
    TimeoutReached,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LitValue {
    True,
    False,
    DontCare,
}

pub trait SolverImpl {
    fn new() -> Self;

    fn add_literal(&mut self, lit: Lit);

    fn assume(&mut self, lit: Lit);

    fn solve(&mut self) -> SolveResult;
    
    fn solve_with_timeout(&mut self, timeout: Duration) -> SolveWithTimeoutResult;

    fn val(&mut self, lit: Lit) -> LitValue;
}

pub struct Solver<T: SolverImpl> {
    pub implementation: T,
    allocator: RangeFrom<u32>,
}

impl<T: SolverImpl> Solver<T> {
    pub fn new() -> Self { Self { implementation: T::new(), allocator: 1.. } }

    pub fn new_lit(&mut self) -> Lit { Lit::new(self.allocator.next().unwrap()) }

    pub fn add_literal(&mut self, lit: Lit) { self.implementation.add_literal(lit); }

    pub fn add_clause(&mut self, clause: impl IntoIterator<Item=Lit>) {
        for lit in clause {
            self.add_literal(lit);
        }
        self.add_literal(Lit { id: 0, negated: false });
    }

    pub fn assume(&mut self, lit: Lit) { self.implementation.assume(lit) }

    pub fn solve(&mut self) -> SolveResult { self.implementation.solve() }
    
    pub fn solve_with_timeout(&mut self, timeout: Duration) -> SolveWithTimeoutResult { self.implementation.solve_with_timeout(timeout) }

    pub fn val(&mut self, lit: Lit) -> LitValue { self.implementation.val(lit) }

    pub fn at_least_one(&mut self, lits: &[Lit]) { self.add_clause(lits.iter().cloned()) }

    pub fn at_least_k(&mut self, strat: AtLeastKStrategy, lits: &[Lit], k: u32) { strat.encode(lits, k, self) }

    pub fn at_most_one(&mut self, strat: AtMostOneStrategy, lits: &[Lit]) { strat.encode(lits, self) }

    pub fn at_most_k(&mut self, strat: AtMostKStrategy, lits: &[Lit], k: u32) { strat.encode(lits, k, self) }

    pub fn exactly_k(&mut self, strat: ExactlyKStrategy, lits: &[Lit], k: u32) { strat.encode(lits, k, self) }
}
