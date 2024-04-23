use std::fs;
use std::fs::DirEntry;
use criterion::Criterion;
use practical_sat::ex1::find_k;
use practical_sat::ex1::graph::Graph;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut paths = fs::read_dir("assets/ex1/coloring")
        .unwrap()
        .map(|item| item.unwrap())
        .collect::<Vec<DirEntry>>();
    paths.sort_by_key(|a| a.file_name());
    
    println!("{paths:#?}");

    let mut group = c.benchmark_group("coloring");

    for path in paths {
        group.bench_function(path.file_name().to_str().unwrap(), |b| b.iter(|| {
            let graph = Graph::parse_dimacs(path.path());

            find_k(graph);
        }));
    }
    group.finish();
}
