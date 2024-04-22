use std::path::Path;

pub mod sequential;
pub mod vecvec;
pub mod literal;

pub trait Cnf {
    fn new() -> Self;
    fn from_dimacs(path: impl AsRef<Path>) -> Self;
}
