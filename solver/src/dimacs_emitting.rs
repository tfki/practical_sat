use std::fmt::Write;
use std::time::Duration;
use crate::{ipasir, LitValue, SolveResult, SolverImpl, SolveWithTimeoutResult};
use crate::literal::Lit;

pub struct Solver {
    actual_solver: ipasir::Solver,

    literals: Vec<Lit>,
    comments: Vec<String>,
}

impl Solver {
    pub fn get_dimacs(&self) -> String {
        let num_clauses = self.literals.iter().filter(|lit| lit.id == 0).count();
        let num_vars = {
            let mut known_vars = vec![];

            for lit in &self.literals {
                if lit.id == 0 { continue; }

                if !known_vars.contains(&lit.id) {
                    known_vars.push(lit.id);
                }
            }

            known_vars.len()
        };

        let mut result = String::new();
        writeln!(result, "p cnf {num_vars} {num_clauses}").unwrap();

        for lit in &self.literals {
            if lit.id == 0 {
                writeln!(result, "0").unwrap();
            } else if lit.negated {
                write!(result, "-{} ", lit.id).unwrap();
            } else {
                write!(result, "{} ", lit.id).unwrap();
            }
        }

        for comment in &self.comments {
            writeln!(result, "c {comment}").unwrap();
        }

        result
    }

    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }
}

impl SolverImpl for Solver {
    fn new() -> Self {
        Self {
            actual_solver: ipasir::Solver::new(),
            literals: vec![],
            comments: vec![],
        }
    }

    fn add_literal(&mut self, lit: Lit) {
        self.actual_solver.add_literal(lit);

        self.literals.push(lit);
    }

    fn assume(&mut self, _lit: Lit) {
        unimplemented!()
    }

    fn solve(&mut self) -> SolveResult {
        self.actual_solver.solve()
    }

    fn solve_with_timeout(&mut self, timeout: Duration) -> SolveWithTimeoutResult { self.actual_solver.solve_with_timeout(timeout) }

    fn val(&mut self, lit: Lit) -> LitValue {
        self.actual_solver.val(lit)
    }
}
