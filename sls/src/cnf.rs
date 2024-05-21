use std::io::{BufRead, BufReader};
use std::path::Path;

use solver::literal::Lit;

#[derive(Debug, Clone)]
pub struct Cnf {
    pub clauses: Vec<Vec<Lit>>,
    pub max_var_id: u32,
}

impl Cnf {
    pub fn from_dimacs(path: impl AsRef<Path>) -> Self {
        let reader = BufReader::new(std::fs::File::open(path).unwrap());

        let mut cnf = Cnf { clauses: Vec::new(), max_var_id: 0 };

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with('p') | line.starts_with('c') {
                continue;
            } else {
                cnf.clauses.push(vec![]);
                line.split_ascii_whitespace()
                    .map(|lit_str| Lit::from(lit_str.parse::<i32>().unwrap()))
                    .for_each(|lit| {
                        if lit.var.id != 0 {
                            cnf.max_var_id = cnf.max_var_id.max(lit.var.id);
                            cnf.clauses.last_mut().unwrap().push(lit);
                        }
                    });
            }
        }

        cnf
    }
}
