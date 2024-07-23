use std::env;

use solver::literal::Lit;

fn main() {
    assert_eq!(env::args().count(), 3);

    let file_cnf1 = env::args().nth(1).unwrap();
    let file_cnf2 = env::args().nth(2).unwrap();

    let cnf1 = sls::cnf::Cnf::from_dimacs(file_cnf1);
    let cnf2 = sls::cnf::Cnf::from_dimacs(file_cnf2);

    let mut allocator = (cnf1.max_var_id.max(cnf2.max_var_id)) + 1..;

    let (new_cnf1, cnf1_repr) = {
        // new cnf 1
        let mut tseitin_vars_clauses_cnf1 = vec![];

        let mut new_cnf1 = cnf1.clauses.iter().flat_map(|clause| {
            let representative = Lit::from(allocator.next().unwrap() as i32);
            tseitin_vars_clauses_cnf1.push(representative);
            let mut results = vec![];

            let mut x_impl_c = clause.clone();
            x_impl_c.push(-representative);

            results.push(x_impl_c);

            for lit in clause {
                results.push(vec![-*lit, representative]);
            }

            results
        }).collect::<Vec<Vec<Lit>>>();

        let cnf1_representative = Lit::from(allocator.next().unwrap() as i32);
        for tseitin_var_clause in &tseitin_vars_clauses_cnf1 {
            new_cnf1.push(vec![*tseitin_var_clause, -cnf1_representative]);
        }
        tseitin_vars_clauses_cnf1 = tseitin_vars_clauses_cnf1.into_iter().map(|lit| -lit).collect();
        tseitin_vars_clauses_cnf1.push(cnf1_representative);
        new_cnf1.push(tseitin_vars_clauses_cnf1);

        (new_cnf1, cnf1_representative)
    };

    let (mut new_cnf2, cnf2_repr) = {
        // new cnf 2
        let mut tseitin_vars_clauses_cnf2 = vec![];

        let mut new_cnf2 = cnf2.clauses.iter().flat_map(|clause| {
            let representative = Lit::from(allocator.next().unwrap() as i32);
            tseitin_vars_clauses_cnf2.push(representative);
            let mut results = vec![];

            let mut x_impl_c = clause.clone();
            x_impl_c.push(-representative);

            results.push(x_impl_c);

            for lit in clause {
                results.push(vec![-*lit, representative]);
            }

            results
        }).collect::<Vec<Vec<Lit>>>();

        let cnf2_representative = Lit::from(allocator.next().unwrap() as i32);
        for tseitin_var_clause in &tseitin_vars_clauses_cnf2 {
            new_cnf2.push(vec![*tseitin_var_clause, -cnf2_representative]);
        }
        tseitin_vars_clauses_cnf2 = tseitin_vars_clauses_cnf2.into_iter().map(|lit| -lit).collect();
        tseitin_vars_clauses_cnf2.push(cnf2_representative);
        new_cnf2.push(tseitin_vars_clauses_cnf2);

        (new_cnf2, cnf2_representative)
    };

    let mut joined_cnf = new_cnf1;
    joined_cnf.append(&mut new_cnf2);

    joined_cnf.push(vec![cnf1_repr, cnf2_repr]);
    joined_cnf.push(vec![-cnf1_repr, -cnf2_repr]);

    println!("p cnf {} {}", allocator.next().unwrap() - 1, joined_cnf.len());
    for clause in joined_cnf {
        for lit in clause {
            print!("{} ", i32::from(lit));
        }
        println!("0");
    }
}
