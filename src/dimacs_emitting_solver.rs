use std::fmt::Write;

use crate::cnf::literal::{Literal, Variable};
use crate::solver::{SolveResult, VariableValue};

pub struct Solver {
    actual_solver: crate::solver::Solver,

    literals: Vec<Literal>,
    comments: Vec<String>,
}


impl Solver {
    pub fn new() -> Self {
        Self {
            actual_solver: crate::solver::Solver::new(),
            literals: vec![],
            comments: vec![],
        }
    }

    pub fn get_dimacs(&self) -> String {
        let num_clauses = self.literals.iter().filter(|lit| lit.var.id == 0).count();
        let num_vars = {
            let mut known_vars = vec![];

            for lit in &self.literals {
                if lit.var.id == 0 { continue; }

                if !known_vars.contains(&lit.var) {
                    known_vars.push(lit.var);
                }
            }

            known_vars.len()
        };

        let mut result = String::new();
        writeln!(result, "p cnf {num_vars} {num_clauses}").unwrap();

        for lit in &self.literals {
            if lit.var.id == 0 {
                writeln!(result, "0 ").unwrap();
            } else {
                if lit.negated {
                    write!(result, "- ").unwrap();
                }
                write!(result, "{} ", lit.var.id).unwrap();
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

    pub fn add_literal(&mut self, lit: Literal) {
        self.actual_solver.add_literal(lit);

        self.literals.push(lit);
    }

    pub fn add_clause(&mut self, clause: &[Literal]) {
        self.actual_solver.add_clause(clause);

        clause.iter().for_each(|lit| self.literals.push(*lit));
        self.literals.push(Literal::clause_end());
    }

    pub fn solve(mut self) -> SolveResult {
        self.actual_solver.solve()
    }

    pub fn val(&mut self, var: Variable) -> VariableValue {
        self.actual_solver.val(var)
    }

    pub fn set_terminate<F>(&mut self, cb: F)
        where
            F: 'static + FnMut() -> bool,
    {
        self.actual_solver.set_terminate(cb)
    }

    pub fn at_least_one(&mut self, lits: &[Literal]) {
        self.add_clause(lits);
    }

    pub fn at_most_one_pairwise(&mut self, lits: &[Literal]) {
        self.actual_solver.at_most_one_pairwise(lits)
    }
}
