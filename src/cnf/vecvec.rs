use std::ffi::c_int;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::cnf::Cnf;
use crate::cnf::literal::Literal;

pub struct VecVecCnf {
    clauses: Vec<Vec<Literal>>,
}

impl Cnf for VecVecCnf {
    fn new() -> Self {
        Self { clauses: vec![] }
    }

    fn from_dimacs(path: impl AsRef<Path>) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let mut lines = reader.lines();

        let first_line = lines.next().unwrap().unwrap();
        let mut first_line_split = first_line.split_ascii_whitespace();

        // ignore 'p'
        first_line_split.next().unwrap();

        // ignore 'cnf'
        first_line_split.next().unwrap();

        // ignore number of variables
        first_line_split.next().unwrap().parse::<usize>().unwrap();
        let num_clauses = first_line_split.next().unwrap().parse::<usize>().unwrap();

        let mut result = VecVecCnf::new();
        result.clauses.reserve(num_clauses);

        for line in lines {
            let clause =
                line.unwrap()
                    .split_ascii_whitespace()
                    .map(str::parse::<c_int>)
                    .map(Result::unwrap)
                    .map(Literal::from)
                    .collect();
            result.clauses.push(clause);
        }

        result
    }
}
