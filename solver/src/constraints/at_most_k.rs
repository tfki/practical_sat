use crate::{Solver, SolverImpl};
use crate::constraints::seq_counter;
use crate::literal::Lit;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AtMostKStrategy {
    SequentialCounter,
}

impl AtMostKStrategy {
    pub fn encode<T: SolverImpl>(self, lits: &[Lit], k: u32, solver: &mut Solver<T>) {
        match self {
            AtMostKStrategy::SequentialCounter => {
                if k > lits.len() as u32
                    || lits.is_empty() {
                    return;
                }

                let last_layer_outputs = seq_counter(lits, solver, k + 1);

                if let Some(x) = last_layer_outputs.get(k as usize) {
                    solver.add_clause([-*x]);
                }
            }
        }
    }
}

macro_rules! gen_test {
    ($strat_ident: ident, $strat_enum_const:expr) => {
        #[cfg(test)]
        mod $strat_ident {
            use crate::{AtMostKStrategy, ipasir, LitValue, Solver, SolveResult};
            use crate::literal::Lit;
            #[test]
            fn k_greater_than_number_of_lits() {
                let num_lits = 3;
                for i in (num_lits + 1)..100 {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
                    
                    solver.at_most_k($strat_enum_const, &lits, i as u32);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                }
            }
            #[test]
            fn k_not_zero_smaller_than_number_of_lits() {
                let num_lits = 6;
                for i in 1..(num_lits - 1) {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
                    
                    solver.at_most_k($strat_enum_const, &lits, i as u32);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                    let trues = lits.iter()
                    .map(|lit| solver.val(*lit))
                    .filter(|val| *val == LitValue::True)
                    .count();
                    assert!(trues <= i);
                }
            }
            #[test]
            fn k_zero() {
                let num_lits = 6;
                for i in 0..num_lits {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
                    
                    solver.at_most_k($strat_enum_const, &lits[0..i], 0);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                    assert!(lits.iter().map(|lit| solver.val(*lit)).all(|lit| lit == LitValue::False));
                }
            }
            #[test]
            fn no_lits() {
                {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    solver.at_most_k($strat_enum_const, &[], 0);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                }
                for k in 1..100 {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    solver.at_most_k($strat_enum_const, &[], k);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                }
            }
        }
    }
}
gen_test!(sequential_counter, AtMostKStrategy::SequentialCounter);
