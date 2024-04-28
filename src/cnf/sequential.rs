use std::ffi::c_int;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::cnf::Cnf;
use crate::cnf::literal::Lit;

enum Item {
    ClauseSeparator,
    Literal(Lit),
}

impl From<Lit> for Item {
    fn from(value: Lit) -> Self {
        Item::Literal(value)
    }
}

pub struct SequentialCnf {
    items: Vec<Item>,
}

impl Cnf for SequentialCnf {
    fn new() -> Self {
        Self { items: vec![] }
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

        let mut result = SequentialCnf::new();
        // 3 * num clauses is a heuristic
        result.items.reserve(3 * num_clauses);

        for line in lines {
            line.unwrap()
                    .split_ascii_whitespace()
                    .map(str::parse::<c_int>)
                    .map(Result::unwrap)
                    .map(Lit::from)
                    .map(Item::from)
                    .for_each(|item| result.items.push(item));
            
            result.items.push(Item::ClauseSeparator);
        }

        result
    }
}
