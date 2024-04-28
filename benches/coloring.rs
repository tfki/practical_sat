use std::path::Path;
use criterion::Criterion;
use practical_sat::ex1::coloring::bitvec_incremental;
use practical_sat::ex1::coloring::graph::Graph;
use practical_sat::util::Timer;

pub fn criterion_benchmark(c: &mut Criterion) {
    let paths = vec![
        "assets/ex1/coloring/anna.col",
        "assets/ex1/coloring/david.col",
        "assets/ex1/coloring/huck.col",
        "assets/ex1/coloring/jean.col",
        "assets/ex1/coloring/le450_5a.col",
        "assets/ex1/coloring/le450_5b.col",
        "assets/ex1/coloring/le450_5c.col",
        "assets/ex1/coloring/le450_5d.col",
        "assets/ex1/coloring/miles250.col",
        "assets/ex1/coloring/myciel3.col",
        "assets/ex1/coloring/myciel4.col",
    ];

    let mut group = c.benchmark_group("coloring");
    group.sample_size(10);

    for path in paths {
        let path = Path::new(path);
        group.bench_function(format!("coloring-{}", path.file_name().unwrap().to_str().unwrap()), |b| b.iter(|| {
            let graph = Graph::parse_dimacs(path);

            bitvec_incremental::find_k(graph, Timer::new_infinite());
        }));
    }
    group.finish();
}
