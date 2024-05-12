
macro_rules! gen_test {
    ($path:expr, $name:ident, $f:expr) => {

        #[test]
        fn $name() {
            use std::path::Path;
            use crate::Sudoku;
            use std::time::Duration;
            use solver::SatProblemResult;

            let path_string = String::from($path);
            let path = Path::new(&path_string);
            let input = Sudoku::parse(path);

            let solution = $f(&input, Duration::from_secs(60));
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert!(solution.is_permutation_of(&input));
                assert!(solution.finished_and_correct());
            }
        }
    };
}

mod naive_one_hot {
    gen_test! {"assets/puzzle03a.sudoku", puzzle03a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle03b.sudoku", puzzle03b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle04a.sudoku", puzzle04a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle04b.sudoku", puzzle04b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle05a.sudoku", puzzle05a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle05b.sudoku", puzzle05b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle06a.sudoku", puzzle06a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle06b.sudoku", puzzle06b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle07a.sudoku", puzzle07a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle07b.sudoku", puzzle07b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle08a.sudoku", puzzle08a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle08b.sudoku", puzzle08b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle09a.sudoku", puzzle09a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle09b.sudoku", puzzle09b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle10a.sudoku", puzzle10a, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle10b.sudoku", puzzle10b, crate::naive_one_hot::find_solution}
    gen_test! {"assets/puzzle11a.sudoku", puzzle11a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle11b.sudoku", puzzle11b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle12a.sudoku", puzzle12a, crate::smart_one_hot::find_solution}
}

mod smart_one_hot {
    gen_test! {"assets/puzzle03a.sudoku", puzzle03a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle03b.sudoku", puzzle03b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle04a.sudoku", puzzle04a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle04b.sudoku", puzzle04b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle05a.sudoku", puzzle05a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle05b.sudoku", puzzle05b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle06a.sudoku", puzzle06a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle06b.sudoku", puzzle06b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle07a.sudoku", puzzle07a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle07b.sudoku", puzzle07b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle08a.sudoku", puzzle08a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle08b.sudoku", puzzle08b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle09a.sudoku", puzzle09a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle09b.sudoku", puzzle09b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle10a.sudoku", puzzle10a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle10b.sudoku", puzzle10b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle11a.sudoku", puzzle11a, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle11b.sudoku", puzzle11b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle12a.sudoku", puzzle12a, crate::smart_one_hot::find_solution}
    // gen_test! {"assets/puzzle12b.sudoku", puzzle12b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle13a.sudoku", puzzle13a, crate::smart_one_hot::find_solution}
    // gen_test! {"assets/puzzle13b.sudoku", puzzle13b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle14a.sudoku", puzzle14a, crate::smart_one_hot::find_solution}
    // gen_test! {"assets/puzzle14b.sudoku", puzzle14b, crate::smart_one_hot::find_solution}
    gen_test! {"assets/puzzle15a.sudoku", puzzle15a, crate::smart_one_hot::find_solution}
    // gen_test! {"assets/puzzle15b.sudoku", puzzle15b, crate::smart_one_hot::find_solution}
}

mod naive_log_encoding {
    gen_test! {"assets/puzzle03a.sudoku", puzzle03a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle03b.sudoku", puzzle03b, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle04a.sudoku", puzzle04a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle04b.sudoku", puzzle04b, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle05a.sudoku", puzzle05a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle05b.sudoku", puzzle05b, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle06a.sudoku", puzzle06a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle06b.sudoku", puzzle06b, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle07a.sudoku", puzzle07a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle07b.sudoku", puzzle07b, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle08a.sudoku", puzzle08a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle08b.sudoku", puzzle08b, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle09a.sudoku", puzzle09a, crate::naive_log::find_solution}
    gen_test! {"assets/puzzle09b.sudoku", puzzle09b, crate::naive_log::find_solution}
    // gen_test! {"assets/puzzle10a.sudoku", puzzle10a, crate::naive_log::find_solution}
    // gen_test! {"assets/puzzle10b.sudoku", puzzle10b, crate::naive_log::find_solution}
    // gen_test! {"assets/puzzle11a.sudoku", puzzle11a, crate::naive_log::find_solution}
    // gen_test! {"assets/puzzle11b.sudoku", puzzle11b, crate::naive_log::find_solution} // ram hogger
    // gen_test! {"assets/puzzle12a.sudoku", puzzle12a, crate::naive_log::find_solution}
}
