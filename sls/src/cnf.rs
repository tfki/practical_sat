use std::io::{BufRead, BufReader};
use std::path::Path;
use solver::literal::Lit;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LitOrClauseEnd {
    Lit(Lit),
    ClauseEnd,
}

#[derive(Debug, Clone)]
pub struct Cnf {
    pub inner: Vec<LitOrClauseEnd>,
    pub max_var_id: u32,
}

impl Cnf {
    pub fn from_dimacs(path: impl AsRef<Path>) -> Self {
        let reader = BufReader::new(std::fs::File::open(path).unwrap());

        let mut cnf = Cnf { inner: Vec::new(), max_var_id: 0 };

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with('p') | line.starts_with('c') {
                continue;
            } else {
                line.split_ascii_whitespace()
                    .map(|lit_str| Lit::from(lit_str.parse::<i32>().unwrap()))
                    .for_each(|lit| {
                        if lit.id != 0 {
                            cnf.max_var_id = cnf.max_var_id.max(lit.id);
                            cnf.inner.push(LitOrClauseEnd::Lit(lit));
                        }
                    });
                cnf.inner.push(LitOrClauseEnd::ClauseEnd);
            }
        }

        cnf
    }
}
