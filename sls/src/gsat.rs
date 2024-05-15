use solver::literal::Lit;

use crate::assignment::Assignment;
use crate::cnf::{Cnf, LitOrClauseEnd};

#[inline(never)]
pub fn solve(cnf: Cnf) -> Option<Assignment> {
    let max_tries = 1000000;
    let max_flips = 2 * cnf.max_var_id;

    let mut buffer = vec![];

    let mut assignment = Assignment::new_random(cnf.max_var_id as usize + 1);
    for _ in 0..max_tries {
        for _ in 0..max_flips {
            match find_var_to_flip(&cnf, &assignment, &mut buffer) {
                FindVarToFlipResult::FoundVar(flip_id) => {
                    assignment[flip_id as usize] = !assignment[flip_id as usize];
                }
                FindVarToFlipResult::FormulaAlreadySat => {
                    return Some(assignment);
                }
            }
        }
        assignment.randomize();
    }

    None
}

enum FindVarToFlipResult {
    FoundVar(u32),
    FormulaAlreadySat,
}

#[inline(never)]
fn find_var_to_flip(cnf: &Cnf, assignment: &Assignment, flip_unsat_counter: &mut Vec<u32>) -> FindVarToFlipResult {
    flip_unsat_counter.resize(cnf.max_var_id as usize + 1, 0);
    flip_unsat_counter.fill(0);

    enum LitFlipCanUnsat {
        DontKnow,
        Yes(Lit),
        No,
    }
    impl LitFlipCanUnsat {
        pub fn is_dont_know(&self) -> bool {
            matches!(self, LitFlipCanUnsat::DontKnow)
        }
    }

    let mut all_clauses_sat = true;

    let mut lit_flip_can_unsat_clause = LitFlipCanUnsat::DontKnow;
    let mut this_clause_sat = false;
    for item in &cnf.inner {
        match item {
            LitOrClauseEnd::Lit(lit) => {
                match (lit.negated, assignment[lit.id as usize]) {
                    (true, true) | (false, false) => {} // lit evaluates to false
                    (false, true) | (true, false) => {
                        // lit evaluates to true
                        this_clause_sat = true;

                        if lit_flip_can_unsat_clause.is_dont_know() {
                            lit_flip_can_unsat_clause = LitFlipCanUnsat::Yes(*lit);
                        } else {
                            lit_flip_can_unsat_clause = LitFlipCanUnsat::No;
                        }
                    }
                }
            }
            LitOrClauseEnd::ClauseEnd => {
                if !this_clause_sat {
                    all_clauses_sat = false;
                }
                this_clause_sat = false;

                if let LitFlipCanUnsat::Yes(lit) = lit_flip_can_unsat_clause {
                    flip_unsat_counter[lit.id as usize] += 1;
                }
                lit_flip_can_unsat_clause = LitFlipCanUnsat::DontKnow;
            }
        }
    }

    if all_clauses_sat {
        FindVarToFlipResult::FormulaAlreadySat
    } else {
        flip_unsat_counter
            .into_iter()
            .enumerate()
            .skip(1)
            .min_by_key(|(_, num)| **num)
            .map(|(id, _)| FindVarToFlipResult::FoundVar(id as u32))
            .unwrap()
    }
}
