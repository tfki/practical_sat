mod single {
    mod sat {
        use solver::SatProblemResult;

        use crate::{CellContent, Input, naive2};

        #[test]
        pub fn i() {
            let solution = naive2::solve(Input {
                width: 1,
                height: 4,
                num_i: 1,
                num_t: 0,
                num_l: 0,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 1);
                assert_eq!(solution.grid[0].len(), 4);
                for x in 0..1 {
                    for y in 0..3 {
                        assert!(matches!(solution.grid[x][y], Some(CellContent::I)));
                    }
                }
            }
        }

        #[test]
        pub fn t() {
            let solution = naive2::solve(Input {
                width: 3,
                height: 2,
                num_i: 0,
                num_t: 1,
                num_l: 0,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 3);
                assert_eq!(solution.grid[0].len(), 2);

                assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[0][1], None));
                assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][1], None));
            }
        }

        #[test]
        pub fn l() {
            let solution = naive2::solve(Input {
                width: 3,
                height: 2,
                num_i: 0,
                num_t: 0,
                num_l: 1,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 3);
                assert_eq!(solution.grid[0].len(), 2);

                assert!(matches!(solution.grid[0][0], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][0], None));
                assert!(matches!(solution.grid[2][0], None));
                assert!(matches!(solution.grid[0][1], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::L)));
                assert!(matches!(solution.grid[2][1], Some(CellContent::L)));
            }
        }

        #[test]
        pub fn s() {
            let solution = naive2::solve(Input {
                width: 3,
                height: 2,
                num_i: 0,
                num_t: 0,
                num_l: 0,
                num_s: 1,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 3);
                assert_eq!(solution.grid[0].len(), 2);

                assert!(matches!(solution.grid[0][0], None));
                assert!(matches!(solution.grid[1][0], Some(CellContent::S)));
                assert!(matches!(solution.grid[2][0], Some(CellContent::S)));
                assert!(matches!(solution.grid[0][1], Some(CellContent::S)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::S)));
                assert!(matches!(solution.grid[2][1], None));
            }
        }

        #[test]
        pub fn o() {
            let solution = naive2::solve(Input {
                width: 2,
                height: 2,
                num_i: 0,
                num_t: 0,
                num_l: 0,
                num_s: 0,
                num_o: 1,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 2);
                assert_eq!(solution.grid[0].len(), 2);

                assert!(matches!(solution.grid[0][0], Some(CellContent::O)));
                assert!(matches!(solution.grid[1][0], Some(CellContent::O)));
                assert!(matches!(solution.grid[0][1], Some(CellContent::O)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::O)));
            }
        }
    }

    mod unsat {
        use solver::SatProblemResult;

        use crate::{Input, naive2};

        #[test]
        pub fn i() {
            let solution = naive2::solve(Input {
                width: 1,
                height: 3,
                num_i: 1,
                num_t: 0,
                num_l: 0,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Unsat));
        }

        #[test]
        pub fn t() {
            let solution = naive2::solve(Input {
                width: 2,
                height: 2,
                num_i: 0,
                num_t: 1,
                num_l: 0,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Unsat));
        }

        #[test]
        pub fn l() {
            let solution = naive2::solve(Input {
                width: 2,
                height: 2,
                num_i: 0,
                num_t: 0,
                num_l: 1,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Unsat));
        }

        #[test]
        pub fn s() {
            let solution = naive2::solve(Input {
                width: 2,
                height: 2,
                num_i: 0,
                num_t: 0,
                num_l: 0,
                num_s: 1,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Unsat));
        }

        #[test]
        pub fn o() {
            let solution = naive2::solve(Input {
                width: 1,
                height: 2,
                num_i: 0,
                num_t: 0,
                num_l: 0,
                num_s: 0,
                num_o: 1,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Unsat));
        }
    }
}

mod pairs {
    mod two_of_a_kind {
        mod sat {
            use solver::SatProblemResult;

            use crate::{CellContent, Input, naive2};

            #[test]
            pub fn s() {
                let solution = naive2::solve(Input {
                    width: 5,
                    height: 2,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 2,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 5);
                    assert_eq!(solution.grid[0].len(), 2);

                    assert!(matches!(solution.grid[0][0], None));
                    assert!(matches!(solution.grid[1][0], Some(CellContent::S)));
                    assert!(matches!(solution.grid[2][0], Some(CellContent::S)));
                    assert!(matches!(solution.grid[3][0], Some(CellContent::S)));
                    assert!(matches!(solution.grid[4][0], Some(CellContent::S)));

                    assert!(matches!(solution.grid[0][1], Some(CellContent::S)));
                    assert!(matches!(solution.grid[1][1], Some(CellContent::S)));
                    assert!(matches!(solution.grid[2][1], Some(CellContent::S)));
                    assert!(matches!(solution.grid[3][1], Some(CellContent::S)));
                    assert!(matches!(solution.grid[4][1], None));
                }
            }

            #[test]
            pub fn i_horizontal() {
                let solution = naive2::solve(Input {
                    width: 2,
                    height: 4,
                    num_i: 2,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 2);
                    assert_eq!(solution.grid[0].len(), 4);

                    assert!(matches!(solution.grid[0][0], Some(CellContent::I)));
                    assert!(matches!(solution.grid[1][0], Some(CellContent::I)));

                    assert!(matches!(solution.grid[0][1], Some(CellContent::I)));
                    assert!(matches!(solution.grid[1][1], Some(CellContent::I)));

                    assert!(matches!(solution.grid[0][2], Some(CellContent::I)));
                    assert!(matches!(solution.grid[1][2], Some(CellContent::I)));

                    assert!(matches!(solution.grid[0][3], Some(CellContent::I)));
                    assert!(matches!(solution.grid[1][3], Some(CellContent::I)));
                }
            }

            #[test]
            pub fn i_vertical() {
                let solution = naive2::solve(Input {
                    width: 1,
                    height: 8,
                    num_i: 2,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 1);
                    assert_eq!(solution.grid[0].len(), 8);

                    assert!(matches!(solution.grid[0][0], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][1], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][2], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][3], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][4], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][5], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][6], Some(CellContent::I)));
                    assert!(matches!(solution.grid[0][7], Some(CellContent::I)));
                }
            }

            #[test]
            pub fn o_horizontal() {
                let solution = naive2::solve(Input {
                    width: 2,
                    height: 4,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 2,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 2);
                    assert_eq!(solution.grid[0].len(), 4);

                    assert!(matches!(solution.grid[0][0], Some(CellContent::O)));
                    assert!(matches!(solution.grid[1][0], Some(CellContent::O)));

                    assert!(matches!(solution.grid[0][1], Some(CellContent::O)));
                    assert!(matches!(solution.grid[1][1], Some(CellContent::O)));

                    assert!(matches!(solution.grid[0][2], Some(CellContent::O)));
                    assert!(matches!(solution.grid[1][2], Some(CellContent::O)));

                    assert!(matches!(solution.grid[0][3], Some(CellContent::O)));
                    assert!(matches!(solution.grid[1][3], Some(CellContent::O)));
                }
            }

            #[test]
            pub fn o_vertical() {
                let solution = naive2::solve(Input {
                    width: 4,
                    height: 2,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 2,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 4);
                    assert_eq!(solution.grid[0].len(), 2);

                    assert!(matches!(solution.grid[0][0], Some(CellContent::O)));
                    assert!(matches!(solution.grid[1][0], Some(CellContent::O)));
                    assert!(matches!(solution.grid[2][0], Some(CellContent::O)));
                    assert!(matches!(solution.grid[3][0], Some(CellContent::O)));

                    assert!(matches!(solution.grid[0][1], Some(CellContent::O)));
                    assert!(matches!(solution.grid[1][1], Some(CellContent::O)));
                    assert!(matches!(solution.grid[2][1], Some(CellContent::O)));
                    assert!(matches!(solution.grid[3][1], Some(CellContent::O)));
                }
            }

            #[test]
            pub fn l() {
                let solution = naive2::solve(Input {
                    width: 4,
                    height: 3,
                    num_i: 0,
                    num_t: 0,
                    num_l: 2,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 4);
                    assert_eq!(solution.grid[0].len(), 3);

                    assert!(matches!(solution.grid[0][0], None));
                    assert!(matches!(solution.grid[1][0], Some(CellContent::L)));
                    assert!(matches!(solution.grid[2][0], None));
                    assert!(matches!(solution.grid[3][0], None));

                    assert!(matches!(solution.grid[0][1], Some(CellContent::L)));
                    assert!(matches!(solution.grid[1][1], Some(CellContent::L)));
                    assert!(matches!(solution.grid[2][1], Some(CellContent::L)));
                    assert!(matches!(solution.grid[3][1], Some(CellContent::L)));

                    assert!(matches!(solution.grid[0][2], Some(CellContent::L)));
                    assert!(matches!(solution.grid[1][2], Some(CellContent::L)));
                    assert!(matches!(solution.grid[2][2], Some(CellContent::L)));
                    assert!(matches!(solution.grid[3][2], None));
                }
            }

            #[test]
            pub fn t() {
                let solution = naive2::solve(Input {
                    width: 5,
                    height: 3,
                    num_i: 0,
                    num_t: 2,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Sat(_)));

                if let SatProblemResult::Sat(solution) = solution {
                    assert_eq!(solution.grid.len(), 5);
                    assert_eq!(solution.grid[0].len(), 3);
                    if solution.grid[4][0].is_some() {
                        assert!(matches!(solution.grid[0][0], None));
                        assert!(matches!(solution.grid[1][0], None));
                        assert!(matches!(solution.grid[2][0], Some(CellContent::T)));
                        assert!(matches!(solution.grid[3][0], Some(CellContent::T)));
                        assert!(matches!(solution.grid[4][0], Some(CellContent::T)));

                        assert!(matches!(solution.grid[0][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[2][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[3][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[4][1], None));

                        assert!(matches!(solution.grid[0][2], None));
                        assert!(matches!(solution.grid[1][2], Some(CellContent::T)));
                        assert!(matches!(solution.grid[2][2], None));
                        assert!(matches!(solution.grid[3][2], None));
                        assert!(matches!(solution.grid[4][2], None));
                    } else {
                        assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
                        assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
                        assert!(matches!(solution.grid[2][0], Some(CellContent::T)));
                        assert!(matches!(solution.grid[3][0], None));
                        assert!(matches!(solution.grid[4][0], None));

                        assert!(matches!(solution.grid[0][1], None));
                        assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[2][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[3][1], Some(CellContent::T)));
                        assert!(matches!(solution.grid[4][1], Some(CellContent::T)));

                        assert!(matches!(solution.grid[0][2], None));
                        assert!(matches!(solution.grid[1][2], None));
                        assert!(matches!(solution.grid[2][2], None));
                        assert!(matches!(solution.grid[3][2], Some(CellContent::T)));
                        assert!(matches!(solution.grid[4][2], None));
                    }
                }
            }
        }

        mod unsat {
            use solver::SatProblemResult;

            use crate::{Input, naive2};

            #[test]
            pub fn s1() {
                let solution = naive2::solve(Input {
                    width: 4,
                    height: 2,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 2,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn s2() {
                let solution = naive2::solve(Input {
                    width: 3,
                    height: 3,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 2,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn i1() {
                let solution = naive2::solve(Input {
                    width: 2,
                    height: 3,
                    num_i: 2,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn i2() {
                let solution = naive2::solve(Input {
                    width: 1,
                    height: 4,
                    num_i: 2,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn o1() {
                let solution = naive2::solve(Input {
                    width: 2,
                    height: 3,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 2,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn o2() {
                let solution = naive2::solve(Input {
                    width: 1,
                    height: 4,
                    num_i: 0,
                    num_t: 0,
                    num_l: 0,
                    num_s: 0,
                    num_o: 2,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn l1() {
                let solution = naive2::solve(Input {
                    width: 3,
                    height: 3,
                    num_i: 0,
                    num_t: 0,
                    num_l: 2,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn l2() {
                let solution = naive2::solve(Input {
                    width: 4,
                    height: 2,
                    num_i: 0,
                    num_t: 0,
                    num_l: 2,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn t1() {
                let solution = naive2::solve(Input {
                    width: 4,
                    height: 3,
                    num_i: 0,
                    num_t: 2,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }

            #[test]
            pub fn t2() {
                let solution = naive2::solve(Input {
                    width: 5,
                    height: 2,
                    num_i: 0,
                    num_t: 2,
                    num_l: 0,
                    num_s: 0,
                    num_o: 0,
                });

                println!("{solution:?}");
                assert!(matches!(solution, SatProblemResult::Unsat));
            }
        }
    }

    mod mixed {
        use solver::SatProblemResult;

        use crate::{CellContent, Input, naive2};

        #[test]
        pub fn s_and_t() {
            let solution = naive2::solve(Input {
                width: 5,
                height: 2,
                num_i: 0,
                num_t: 1,
                num_l: 0,
                num_s: 1,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 5);
                assert_eq!(solution.grid[0].len(), 2);

                assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[3][0], Some(CellContent::S)));
                assert!(matches!(solution.grid[4][0], Some(CellContent::S)));

                assert!(matches!(solution.grid[0][1], None));
                assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][1], Some(CellContent::S)));
                assert!(matches!(solution.grid[3][1], Some(CellContent::S)));
                assert!(matches!(solution.grid[4][1], None));
            }
        }

        #[test]
        pub fn l_and_t() {
            let solution = naive2::solve(Input {
                width: 3,
                height: 3,
                num_i: 0,
                num_t: 1,
                num_l: 1,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 3);
                assert_eq!(solution.grid[0].len(), 3);

                assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][0], Some(CellContent::T)));

                assert!(matches!(solution.grid[0][1], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][1], None));

                assert!(matches!(solution.grid[0][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[2][2], Some(CellContent::L)));
            }
        }

        #[test]
        pub fn l_and_s() {
            let solution = naive2::solve(Input {
                width: 4,
                height: 3,
                num_i: 0,
                num_t: 0,
                num_l: 1,
                num_s: 1,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 4);
                assert_eq!(solution.grid[0].len(), 3);

                assert!(matches!(solution.grid[0][0], None));
                assert!(matches!(solution.grid[1][0], None));
                assert!(matches!(solution.grid[2][0], Some(CellContent::S)));
                assert!(matches!(solution.grid[3][0], Some(CellContent::S)));

                assert!(matches!(solution.grid[0][1], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::S)));
                assert!(matches!(solution.grid[2][1], Some(CellContent::S)));
                assert!(matches!(solution.grid[3][1], None));

                assert!(matches!(solution.grid[0][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[2][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[3][2], None));
            }
        }

        #[test]
        pub fn l_and_o() {
            let solution = naive2::solve(Input {
                width: 3,
                height: 3,
                num_i: 0,
                num_t: 0,
                num_l: 1,
                num_s: 0,
                num_o: 1,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 3);
                assert_eq!(solution.grid[0].len(), 3);

                assert!(matches!(solution.grid[0][0], None));
                assert!(matches!(solution.grid[1][0], Some(CellContent::O)));
                assert!(matches!(solution.grid[2][0], Some(CellContent::O)));

                assert!(matches!(solution.grid[0][1], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::O)));
                assert!(matches!(solution.grid[2][1], Some(CellContent::O)));

                assert!(matches!(solution.grid[0][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[1][2], Some(CellContent::L)));
                assert!(matches!(solution.grid[2][2], Some(CellContent::L)));
            }
        }

        #[test]
        pub fn t_and_i() {
            let solution = naive2::solve(Input {
                width: 3,
                height: 5,
                num_i: 2,
                num_t: 1,
                num_l: 0,
                num_s: 0,
                num_o: 0,
            });

            println!("{solution:?}");
            assert!(matches!(solution, SatProblemResult::Sat(_)));

            if let SatProblemResult::Sat(solution) = solution {
                assert_eq!(solution.grid.len(), 3);
                assert_eq!(solution.grid[0].len(), 5);

                assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][0], Some(CellContent::T)));

                assert!(matches!(solution.grid[0][1], Some(CellContent::I)));
                assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
                assert!(matches!(solution.grid[2][1], Some(CellContent::I)));

                assert!(matches!(solution.grid[0][2], Some(CellContent::I)));
                assert!(matches!(solution.grid[1][2], None));
                assert!(matches!(solution.grid[2][2], Some(CellContent::I)));

                assert!(matches!(solution.grid[0][3], Some(CellContent::I)));
                assert!(matches!(solution.grid[1][3], None));
                assert!(matches!(solution.grid[2][3], Some(CellContent::I)));

                assert!(matches!(solution.grid[0][4], Some(CellContent::I)));
                assert!(matches!(solution.grid[1][4], None));
                assert!(matches!(solution.grid[2][4], Some(CellContent::I)));
            }
        }
    }
}

mod complex {
    use solver::SatProblemResult;

    use crate::{CellContent, Input, naive2};

    #[test]
    pub fn exercise_sheet_example1() {
        let solution = naive2::solve(Input {
            height: 5,
            width: 8,
            num_i: 1,
            num_t: 2,
            num_l: 2,
            num_s: 2,
            num_o: 3,
        });

        println!("{solution:?}");
        assert!(matches!(solution, SatProblemResult::Sat(_)));

        if let SatProblemResult::Sat(solution) = solution {
            assert_eq!(solution.grid.len(), 8);
            assert_eq!(solution.grid[0].len(), 5);

            assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[2][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[3][0], Some(CellContent::S)));
            assert!(matches!(solution.grid[4][0], Some(CellContent::S)));
            assert!(matches!(solution.grid[5][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[6][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[7][0], Some(CellContent::T)));

            assert!(matches!(solution.grid[0][1], Some(CellContent::L)));
            assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
            assert!(matches!(solution.grid[2][1], Some(CellContent::S)));
            assert!(matches!(solution.grid[3][1], Some(CellContent::S)));
            assert!(matches!(solution.grid[4][1], Some(CellContent::S)));
            assert!(matches!(solution.grid[5][1], Some(CellContent::S)));
            assert!(matches!(solution.grid[6][1], Some(CellContent::T)));
            assert!(matches!(solution.grid[7][1], Some(CellContent::I)));

            assert!(matches!(solution.grid[0][2], Some(CellContent::L)));
            assert!(matches!(solution.grid[1][2], Some(CellContent::L)));
            assert!(matches!(solution.grid[2][2], Some(CellContent::L)));
            assert!(matches!(solution.grid[3][2], Some(CellContent::S)));
            assert!(matches!(solution.grid[4][2], Some(CellContent::S)));
            assert!(matches!(solution.grid[5][2], Some(CellContent::O)));
            assert!(matches!(solution.grid[6][2], Some(CellContent::O)));
            assert!(matches!(solution.grid[7][2], Some(CellContent::I)));

            assert!(matches!(solution.grid[0][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[1][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[2][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[3][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[4][3], Some(CellContent::L)));
            assert!(matches!(solution.grid[5][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[6][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[7][3], Some(CellContent::I)));

            assert!(matches!(solution.grid[0][4], Some(CellContent::O)));
            assert!(matches!(solution.grid[1][4], Some(CellContent::O)));
            assert!(matches!(solution.grid[2][4], Some(CellContent::O)));
            assert!(matches!(solution.grid[3][4], Some(CellContent::O)));
            assert!(matches!(solution.grid[4][4], Some(CellContent::L)));
            assert!(matches!(solution.grid[5][4], Some(CellContent::L)));
            assert!(matches!(solution.grid[6][4], Some(CellContent::L)));
            assert!(matches!(solution.grid[7][4], Some(CellContent::I)));
        }
    }


    #[test]
    pub fn exercise_sheet_example2() {
        let solution = naive2::solve(Input {
            height: 5,
            width: 5,
            num_i: 2,
            num_t: 1,
            num_l: 1,
            num_s: 1,
            num_o: 1,
        });

        println!("{solution:?}");
        assert!(matches!(solution, SatProblemResult::Sat(_)));

        if let SatProblemResult::Sat(solution) = solution {
            assert_eq!(solution.grid.len(), 5);
            assert_eq!(solution.grid[0].len(), 5);

            assert!(matches!(solution.grid[0][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[1][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[2][0], Some(CellContent::T)));
            assert!(matches!(solution.grid[3][0], Some(CellContent::S)));
            assert!(matches!(solution.grid[4][0], Some(CellContent::S)));

            assert!(matches!(solution.grid[0][1], Some(CellContent::I)));
            assert!(matches!(solution.grid[1][1], Some(CellContent::T)));
            assert!(matches!(solution.grid[2][1], Some(CellContent::S)));
            assert!(matches!(solution.grid[3][1], Some(CellContent::S)));
            assert!(matches!(solution.grid[4][1], Some(CellContent::I)));

            assert!(matches!(solution.grid[0][2], Some(CellContent::I)));
            assert!(matches!(solution.grid[1][2], None));
            assert!(matches!(solution.grid[2][2], Some(CellContent::O)));
            assert!(matches!(solution.grid[3][2], Some(CellContent::O)));
            assert!(matches!(solution.grid[4][2], Some(CellContent::I)));

            assert!(matches!(solution.grid[0][3], Some(CellContent::I)));
            assert!(matches!(solution.grid[1][3], Some(CellContent::L)));
            assert!(matches!(solution.grid[2][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[3][3], Some(CellContent::O)));
            assert!(matches!(solution.grid[4][3], Some(CellContent::I)));

            assert!(matches!(solution.grid[0][4], Some(CellContent::I)));
            assert!(matches!(solution.grid[1][4], Some(CellContent::L)));
            assert!(matches!(solution.grid[2][4], Some(CellContent::L)));
            assert!(matches!(solution.grid[3][4], Some(CellContent::L)));
            assert!(matches!(solution.grid[4][4], Some(CellContent::I)));
        }
    }
}
