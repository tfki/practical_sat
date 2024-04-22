use std::path::Path;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use practical_sat::cnf::sequential::SequentialCnf;
use practical_sat::cnf::Cnf;
use practical_sat::cnf::vecvec::VecVecCnf;

fn sequential(path: impl AsRef<Path>) -> SequentialCnf {
    SequentialCnf::from_dimacs(path)
}

fn vecvec(path: impl AsRef<Path>) -> VecVecCnf {
    VecVecCnf::from_dimacs(path)
}

fn criterion_benchmark(c: &mut Criterion) {
    let path = Path::new("assets/000a41cdca43be89ed62ea3abf2d0b64-snw_13_9_pre.cnf");
    
    c.bench_function("sequential", |b| b.iter(|| sequential(black_box(path))));
    c.bench_function("vecvec", |b| b.iter(|| vecvec(black_box(path))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
