
macro_rules! gen_test {
    ($path:expr, $name:ident, $f:expr) => {

        #[test]
        fn $name() {
            use std::path::Path;
            use crate::ex1::sudoku::Sudoku;
            use std::time::Duration;
            use crate::SatProblemResult;
            use crate::util::Timer;

            let path_string = String::from($path);
            let path = Path::new(&path_string);
            let input = Sudoku::parse(path);

            let solution = $f(&input, Timer::new(Duration::from_secs(60)));
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert!(solution.is_permutation_of(&input));
                assert!(solution.finished_and_correct());
            }
        }
    };
}

mod naive_one_hot {
    gen_test! {"assets/ex1/sudoku/puzzle03a.sudoku", puzzle03a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle03b.sudoku", puzzle03b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle04a.sudoku", puzzle04a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle04b.sudoku", puzzle04b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle05a.sudoku", puzzle05a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle05b.sudoku", puzzle05b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle06a.sudoku", puzzle06a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle06b.sudoku", puzzle06b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle07a.sudoku", puzzle07a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle07b.sudoku", puzzle07b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle08a.sudoku", puzzle08a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle08b.sudoku", puzzle08b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle09a.sudoku", puzzle09a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle09b.sudoku", puzzle09b, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle10a.sudoku", puzzle10a, crate::ex1::sudoku::naive_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle10b.sudoku", puzzle10b, crate::ex1::sudoku::naive_one_hot::find_solution}
}

mod smart_one_hot {
    gen_test! {"assets/ex1/sudoku/puzzle03a.sudoku", puzzle03a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle03b.sudoku", puzzle03b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle04a.sudoku", puzzle04a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle04b.sudoku", puzzle04b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle05a.sudoku", puzzle05a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle05b.sudoku", puzzle05b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle06a.sudoku", puzzle06a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle06b.sudoku", puzzle06b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle07a.sudoku", puzzle07a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle07b.sudoku", puzzle07b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle08a.sudoku", puzzle08a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle08b.sudoku", puzzle08b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle09a.sudoku", puzzle09a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle09b.sudoku", puzzle09b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle10a.sudoku", puzzle10a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle10b.sudoku", puzzle10b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle11a.sudoku", puzzle11a, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle11b.sudoku", puzzle11b, crate::ex1::sudoku::smart_one_hot::find_solution}
    gen_test! {"assets/ex1/sudoku/puzzle12a.sudoku", puzzle12a, crate::ex1::sudoku::smart_one_hot::find_solution}
}
