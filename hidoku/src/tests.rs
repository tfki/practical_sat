macro_rules! gen_test {
    ($path:expr, $name:ident, $f:expr) => {

        #[test]
        fn $name() {
            use solver::SatProblemResult;
            use crate::hidoku::Hidoku;

            let problem_path = $path;
            let problem_string = std::fs::read_to_string(problem_path).unwrap();
            let hidoku = Hidoku::from(problem_string.clone());
            let solution = $f(hidoku.clone());

            assert!(matches!(solution, SatProblemResult::Sat(_)));
            if let SatProblemResult::Sat(solution) = solution {
                assert!(solution.is_permutation_of(&hidoku));
            }
        }
    };
}

mod naive_w_preprocessor {
    gen_test! {"assets/hidoku_3x3_1", hidoku_3x3_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_2", hidoku_3x3_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_3", hidoku_3x3_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_4", hidoku_3x3_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_5", hidoku_3x3_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_6", hidoku_3x3_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_7", hidoku_3x3_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_8", hidoku_3x3_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_9", hidoku_3x3_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_10", hidoku_3x3_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_1", hidoku_4x4_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_2", hidoku_4x4_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_3", hidoku_4x4_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_4", hidoku_4x4_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_5", hidoku_4x4_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_6", hidoku_4x4_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_7", hidoku_4x4_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_8", hidoku_4x4_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_9", hidoku_4x4_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_10", hidoku_4x4_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_1", hidoku_5x5_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_2", hidoku_5x5_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_3", hidoku_5x5_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_4", hidoku_5x5_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_5", hidoku_5x5_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_6", hidoku_5x5_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_7", hidoku_5x5_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_8", hidoku_5x5_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_9", hidoku_5x5_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_10", hidoku_5x5_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_1", hidoku_6x6_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_2", hidoku_6x6_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_3", hidoku_6x6_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_4", hidoku_6x6_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_5", hidoku_6x6_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_6", hidoku_6x6_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_7", hidoku_6x6_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_8", hidoku_6x6_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_9", hidoku_6x6_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_10", hidoku_6x6_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_1", hidoku_7x7_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_2", hidoku_7x7_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_3", hidoku_7x7_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_4", hidoku_7x7_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_5", hidoku_7x7_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_6", hidoku_7x7_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_7", hidoku_7x7_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_8", hidoku_7x7_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_9", hidoku_7x7_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_10", hidoku_7x7_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_1", hidoku_8x8_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_2", hidoku_8x8_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_3", hidoku_8x8_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_4", hidoku_8x8_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_5", hidoku_8x8_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_6", hidoku_8x8_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_7", hidoku_8x8_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_8", hidoku_8x8_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_9", hidoku_8x8_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_10", hidoku_8x8_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_1", hidoku_9x9_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_2", hidoku_9x9_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_3", hidoku_9x9_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_4", hidoku_9x9_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_5", hidoku_9x9_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_6", hidoku_9x9_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_7", hidoku_9x9_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_8", hidoku_9x9_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_9", hidoku_9x9_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_10", hidoku_9x9_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_1", hidoku_10x10_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_2", hidoku_10x10_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_3", hidoku_10x10_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_4", hidoku_10x10_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_5", hidoku_10x10_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_6", hidoku_10x10_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_7", hidoku_10x10_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_8", hidoku_10x10_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_9", hidoku_10x10_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_10", hidoku_10x10_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_1", hidoku_11x11_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_2", hidoku_11x11_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_3", hidoku_11x11_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_4", hidoku_11x11_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_5", hidoku_11x11_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_6", hidoku_11x11_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_7", hidoku_11x11_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_8", hidoku_11x11_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_9", hidoku_11x11_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_10", hidoku_11x11_10, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_1", hidoku_12x12_1, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_2", hidoku_12x12_2, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_3", hidoku_12x12_3, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_4", hidoku_12x12_4, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_5", hidoku_12x12_5, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_6", hidoku_12x12_6, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_7", hidoku_12x12_7, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_8", hidoku_12x12_8, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_9", hidoku_12x12_9, crate::encodings::naive_w_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_10", hidoku_12x12_10, crate::encodings::naive_w_preprocessor::solve}
}

mod naive_no_preprocessor {
    gen_test! {"assets/hidoku_3x3_1", hidoku_3x3_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_2", hidoku_3x3_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_3", hidoku_3x3_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_4", hidoku_3x3_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_5", hidoku_3x3_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_6", hidoku_3x3_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_7", hidoku_3x3_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_8", hidoku_3x3_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_9", hidoku_3x3_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_3x3_10", hidoku_3x3_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_1", hidoku_4x4_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_2", hidoku_4x4_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_3", hidoku_4x4_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_4", hidoku_4x4_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_5", hidoku_4x4_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_6", hidoku_4x4_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_7", hidoku_4x4_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_8", hidoku_4x4_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_9", hidoku_4x4_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_4x4_10", hidoku_4x4_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_1", hidoku_5x5_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_2", hidoku_5x5_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_3", hidoku_5x5_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_4", hidoku_5x5_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_5", hidoku_5x5_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_6", hidoku_5x5_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_7", hidoku_5x5_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_8", hidoku_5x5_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_9", hidoku_5x5_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_5x5_10", hidoku_5x5_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_1", hidoku_6x6_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_2", hidoku_6x6_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_3", hidoku_6x6_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_4", hidoku_6x6_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_5", hidoku_6x6_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_6", hidoku_6x6_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_7", hidoku_6x6_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_8", hidoku_6x6_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_9", hidoku_6x6_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_6x6_10", hidoku_6x6_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_1", hidoku_7x7_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_2", hidoku_7x7_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_3", hidoku_7x7_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_4", hidoku_7x7_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_5", hidoku_7x7_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_6", hidoku_7x7_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_7", hidoku_7x7_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_8", hidoku_7x7_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_9", hidoku_7x7_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_7x7_10", hidoku_7x7_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_1", hidoku_8x8_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_2", hidoku_8x8_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_3", hidoku_8x8_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_4", hidoku_8x8_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_5", hidoku_8x8_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_6", hidoku_8x8_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_7", hidoku_8x8_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_8", hidoku_8x8_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_9", hidoku_8x8_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_8x8_10", hidoku_8x8_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_1", hidoku_9x9_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_2", hidoku_9x9_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_3", hidoku_9x9_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_4", hidoku_9x9_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_5", hidoku_9x9_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_6", hidoku_9x9_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_7", hidoku_9x9_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_8", hidoku_9x9_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_9", hidoku_9x9_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_9x9_10", hidoku_9x9_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_1", hidoku_10x10_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_2", hidoku_10x10_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_3", hidoku_10x10_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_4", hidoku_10x10_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_5", hidoku_10x10_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_6", hidoku_10x10_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_7", hidoku_10x10_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_8", hidoku_10x10_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_9", hidoku_10x10_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_10x10_10", hidoku_10x10_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_1", hidoku_11x11_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_2", hidoku_11x11_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_3", hidoku_11x11_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_4", hidoku_11x11_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_5", hidoku_11x11_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_6", hidoku_11x11_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_7", hidoku_11x11_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_8", hidoku_11x11_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_9", hidoku_11x11_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_11x11_10", hidoku_11x11_10, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_1", hidoku_12x12_1, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_2", hidoku_12x12_2, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_3", hidoku_12x12_3, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_4", hidoku_12x12_4, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_5", hidoku_12x12_5, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_6", hidoku_12x12_6, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_7", hidoku_12x12_7, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_8", hidoku_12x12_8, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_9", hidoku_12x12_9, crate::encodings::naive_no_preprocessor::solve}
    gen_test! {"assets/hidoku_12x12_10", hidoku_12x12_10, crate::encodings::naive_no_preprocessor::solve}
}
