use criterion::{criterion_group, criterion_main, Criterion};

mod coloring;
mod sudoku;

fn criterion_benchmark(c: &mut Criterion) {
    coloring::criterion_benchmark(c);
    sudoku::criterion_benchmark(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

