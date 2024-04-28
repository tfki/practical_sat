use std::path::Path;

use criterion::Criterion;

use practical_sat::ex1::sudoku::naive_one_hot::find_solution;
use practical_sat::ex1::sudoku::sudoku::Sudoku;

pub fn criterion_benchmark(c: &mut Criterion) {
    let paths = vec![
        "assets/ex1/sudoku/puzzle03a.sudoku",
        "assets/ex1/sudoku/puzzle03b.sudoku",
        "assets/ex1/sudoku/puzzle04a.sudoku",
        "assets/ex1/sudoku/puzzle04b.sudoku",
    ];

    let mut group = c.benchmark_group("sudoku");
    group.sample_size(10);

    for path in paths {
        let path = Path::new(path);
        group.bench_function(format!("sudoku-{}", path.file_name().unwrap().to_str().unwrap()), |b| b.iter(|| {
            let input = Sudoku::parse(path);

            find_solution(&input);
        }));
    }
    group.finish();
}
