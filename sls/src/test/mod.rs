mod gsat;
mod walksat;

macro_rules! gen_test {
    ($name:ident, $file:expr, $f:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            use solver::{ipasir, Solver, SolveResult};
            use solver::literal::Lit;
            use solver::timer::Timer;
            use crate::cnf::Cnf;
            use std::time::Duration;

            let cnf = Cnf::from_dimacs($file);
            let mut solver = Solver::<ipasir::Solver>::new();

            solver.load_dimacs_from_file($file);
            let sls_assignment = $f(cnf, Some(Timer::new(Duration::from_secs(10)))).unwrap();
            for (var, value) in sls_assignment {
                solver.add_clause([Lit { var, negated: !value}]);
            }
            assert!(matches!(solver.solve(), SolveResult::Sat));
        }
    };
}
pub (crate) use gen_test;
