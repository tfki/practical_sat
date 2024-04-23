use criterion::{criterion_group, criterion_main, Criterion};

mod coloring;
mod dimacs_parse;

fn criterion_benchmark(c: &mut Criterion) {
    coloring::criterion_benchmark(c);
    dimacs_parse::criterion_benchmark(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

