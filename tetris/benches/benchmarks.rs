use criterion::{Criterion, criterion_group, criterion_main};
use solver::SatProblemResult;

use tetris::Solution;

fn a() -> SatProblemResult<Solution> {
    let input = tetris::Input {
        width: 10,
        height: 5,
        num_i: 3,
        num_t: 3,
        num_l: 3,
        num_s: 2,
        num_o: 1,
    };
    tetris::naive::solve(input)
}

fn b() -> SatProblemResult<Solution> {
    let input = tetris::Input {
        width: 10,
        height: 8,
        num_i: 4,
        num_t: 4,
        num_l: 4,
        num_s: 4,
        num_o: 3,
    };
    tetris::naive::solve(input)
}

fn c() -> SatProblemResult<Solution> {
    let input = tetris::Input {
        width: 20,
        height: 10,
        num_i: 10,
        num_t: 10,
        num_l: 10,
        num_s: 10,
        num_o: 6,
    };
    tetris::naive::solve(input)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tetris");
    group.sample_size(10);

    group.bench_function("a", |bencher| bencher.iter(a));
    group.bench_function("b", |bencher| bencher.iter(b));
    group.bench_function("c", |bencher| bencher.iter(b));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
