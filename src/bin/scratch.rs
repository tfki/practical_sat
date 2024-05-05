use practical_sat::solver::{ExactlyKStrategy, ipasir, Solver, SolveResult};
use practical_sat::solver::literal::Lit;

fn main() {
    let mut solver = Solver::<ipasir::Solver>::new();

    let lits = (0..10).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
    solver.exactly_k(ExactlyKStrategy::SequentialCounter, &lits[0..5], 2);

    assert!(matches!(solver.solve(), SolveResult::Sat));

    for lit in lits {
        println!("{lit:?} is {:?}", solver.val(lit));
    }
}
