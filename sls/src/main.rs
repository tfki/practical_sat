use sls::cnf::Cnf;
use sls::gsat;

fn main() {
    use solver::{ipasir, Solver, SolveResult};

    let path = "assets/d24e98d3e8f9ce352e81ffc56962eeba-driverlog1_v01i.shuffled-as.sat05-4027.cnf";
    let cnf = Cnf::from_dimacs(path);
    let mut solver = Solver::<ipasir::Solver>::new();

    solver.load_dimacs_from_file(path);
    let sls_assignment = gsat::solve(cnf).unwrap();
    for lit in sls_assignment {
        solver.add_clause([lit]);
    }
    assert!(matches!(solver.solve(), SolveResult::Sat));
}
