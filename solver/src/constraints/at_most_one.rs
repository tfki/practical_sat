use crate::{AtMostKStrategy, Solver, SolverImpl};
use crate::literal::Lit;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AtMostOneStrategy {
    SequentialCounter,
    Pairwise,
}

impl AtMostOneStrategy {
    pub fn encode(self, lits: &[Lit], solver: &mut Solver<impl SolverImpl>) {
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

macro_rules! gen_test {
    ($strat_ident: ident, $strat_enum_const:expr) => {
        #[cfg(test)]
        mod $strat_ident {
            use crate::{AtMostOneStrategy, ipasir, LitValue, Solver, SolveResult};
            use crate::literal::Lit;
            #[test]
            fn many_lits() {
                let num_lits = 6;
                for i in 1..num_lits {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
                    
                    solver.at_most_one($strat_enum_const, &lits[0..i]);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                    assert!(lits.iter().map(|lit| solver.val(*lit)).filter(|lit| *lit == LitValue::True).count() <= 1);
                }
            }
            #[test]
            fn no_lits() {
                let mut solver = Solver::<ipasir::Solver>::new();
                solver.at_most_one($strat_enum_const, &[]);
                assert!(matches!(solver.solve(), SolveResult::Sat));
            }
        }
    }
}
gen_test!(sequential_counter, AtMostOneStrategy::SequentialCounter);
gen_test!(pairwise, AtMostOneStrategy::Pairwise);
