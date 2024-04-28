pub mod cnf;
pub mod solver;
pub mod dimacs_emitting_solver;
pub mod ex1;
pub mod util;

pub enum SatProblemResult<T> {
    Sat(T),
    Unsat,
    Timeout,
}
