use crate::{Solver, SolverImpl};
use crate::constraints::seq_counter;
use crate::literal::Lit;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ExactlyKStrategy {
    SequentialCounter,
}

impl ExactlyKStrategy {
    pub fn encode(self, lits: &[Lit], k: u32, solver: &mut Solver<impl SolverImpl>) {
        match self {
            ExactlyKStrategy::SequentialCounter => {
                if k > lits.len() as u32 {
                    solver.add_clause([]);
                    return;
                }
                if lits.is_empty() && k == 0 {
                    return;
                }

                let last_layer_outputs = seq_counter(lits, solver, k + 1);

                if k > 0 {
                    if let Some(x) = last_layer_outputs.get((k - 1) as usize) {
                        solver.add_clause([*x]);
                    }
                }
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
            use crate::{ExactlyKStrategy, ipasir, LitValue, Solver, SolveResult};
            use crate::literal::Lit;
            #[test]
            fn k_greater_than_number_of_lits() {
                let num_lits = 4;
                for i in (num_lits + 1)..100 {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();

                    solver.exactly_k($strat_enum_const, &lits, i as u32);
                    assert!(matches!(solver.solve(), SolveResult::Unsat));
                }
            }
            #[test]
            fn k_not_zero_up_to_number_of_lits() {
                let num_lits = 6;
                for i in 1..num_lits {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
                    
                    solver.exactly_k($strat_enum_const, &lits, i as u32);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                    let trues = lits.iter()
                    .map(|lit| solver.val(*lit))
                    .filter(|val| *val == LitValue::True)
                    .count();
                    assert!(trues == i);
                }
            }
            #[test]
            fn k_zero() {
                let num_lits = 6;
                for i in 0..num_lits {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    let lits = (0..num_lits).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
                    
                    solver.exactly_k($strat_enum_const, &lits[0..i], 0);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                    assert!(lits.iter().map(|lit| solver.val(*lit)).all(|lit| lit == LitValue::False));
                }
            }
            #[test]
            fn no_lits() {
                {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    solver.exactly_k($strat_enum_const, &[], 0);
                    assert!(matches!(solver.solve(), SolveResult::Sat));
                }
                for k in 1..100 {
                    let mut solver = Solver::<ipasir::Solver>::new();
                    solver.exactly_k($strat_enum_const, &[], k);
                    assert!(matches!(solver.solve(), SolveResult::Unsat));
                }
            }
        }
    }
}
gen_test!(sequential_counter, ExactlyKStrategy::SequentialCounter);
