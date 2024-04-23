use std::path::Path;
use criterion::{black_box, Criterion};
use practical_sat::cnf::sequential::SequentialCnf;
use practical_sat::cnf::Cnf;
use practical_sat::cnf::vecvec::VecVecCnf;

fn sequential(path: impl AsRef<Path>) -> SequentialCnf {
    SequentialCnf::from_dimacs(path)
}

fn vecvec(path: impl AsRef<Path>) -> VecVecCnf {
    VecVecCnf::from_dimacs(path)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dimacs_parse");
    let path = Path::new("assets/000a41cdca43be89ed62ea3abf2d0b64-snw_13_9_pre.cnf");

    group.bench_function("sequential", |b| b.iter(|| sequential(black_box(path))));
    group.bench_function("vecvec", |b| b.iter(|| vecvec(black_box(path))));
    
    group.finish();
}
