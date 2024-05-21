use std::io::{BufRead, BufReader};
use std::ops::RangeFrom;
use std::path::Path;
use std::time::Duration;

pub use constraints::*;
use literal::Lit;

use crate::variable::Var;

pub mod dimacs_emitting;
pub mod ipasir;
pub mod timer;
mod constraints;
pub mod literal;
pub mod variable;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SatProblemResult<T> {
    Sat(T),
    Unsat,
    Timeout,
}

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

impl<T: SolverImpl> Default for Solver<T> {
    fn default() -> Self {
        Self {
            implementation: T::new(),
            allocator: 0..,
        }
    }
}

impl<T: SolverImpl> Solver<T> {
    pub fn new() -> Self { Self { implementation: T::new(), allocator: 1.. } }

    pub fn new_lit(&mut self) -> Lit { Lit::new(self.allocator.next().unwrap()) }

    pub fn load_dimacs_from_file(&mut self, path: impl AsRef<Path>) {
        let reader = BufReader::new(std::fs::File::open(path).unwrap());

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with('p') | line.starts_with('c') {
                continue;
            } else {
                let mut solver_with_open_clause = std::mem::take(self).start_clause();
                line.split_ascii_whitespace()
                    .map(|lit_str| Lit::from(lit_str.parse::<i32>().unwrap()))
                    .for_each(|lit| {
                        if lit.var.id != 0 {
                            solver_with_open_clause.add_literal(lit);
                        }
                    });
                *self = solver_with_open_clause.end_clause();
            }
        }
    }

    pub fn start_clause(self) -> SolverWithOpenClause<T> {
        SolverWithOpenClause {
            implementation: self.implementation,
            allocator: self.allocator,
        }
    }

    pub fn add_clause(&mut self, clause: impl IntoIterator<Item=Lit>) {
        for lit in clause {
            self.implementation.add_literal(lit);
        }
        self.implementation.add_literal(Lit { var: Var { id: 0 }, negated: false });
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

pub struct SolverWithOpenClause<T: SolverImpl> {
    pub implementation: T,
    allocator: RangeFrom<u32>,
}

impl<T: SolverImpl> SolverWithOpenClause<T> {
    pub fn new_lit(&mut self) -> Lit { Lit::new(self.allocator.next().unwrap()) }

    pub fn add_literal(&mut self, lit: Lit) { self.implementation.add_literal(lit); }

    pub fn end_clause(mut self) -> Solver<T> {
        self.implementation.add_literal(Lit { var: Var { id: 0 }, negated: false });
        Solver { implementation: self.implementation, allocator: self.allocator }
    }
}
