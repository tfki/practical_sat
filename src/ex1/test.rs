use std::path::Path;
use std::time::Duration;
use crate::util::panic_after;
use crate::ex1::find_k;
use crate::ex1::graph::Graph;

macro_rules! gen_test {
    ($path:expr, $name:ident, $k:expr) => {
        #[test]
        fn $name() {
            panic_after(Duration::from_secs(10), || {
                let path_string = String::from($path);
                let path = Path::new(&path_string);
                let graph = Graph::parse_dimacs(path);
                assert!(matches!(find_k(graph, $k), Ok($k)));
            });
        }
    };
}

gen_test! {"assets/ex1/coloring/myciel3.col", myciel3, 4}
gen_test! {"assets/ex1/coloring/myciel4.col", myciel4, 5}
gen_test! {"assets/ex1/coloring/le450_5a.col", le450_5a, 5}
gen_test! {"assets/ex1/coloring/le450_5b.col", le450_5b, 5}
gen_test! {"assets/ex1/coloring/le450_5c.col", le450_5c, 5}
gen_test! {"assets/ex1/coloring/le450_5d.col", le450_5d, 5}
gen_test! {"assets/ex1/coloring/myciel5.col", myciel5, 6}
gen_test! {"assets/ex1/coloring/myciel6.col", myciel6, 7}
gen_test! {"assets/ex1/coloring/myciel7.col", myciel7, 8}
gen_test! {"assets/ex1/coloring/miles250.col", miles250, 8}
gen_test! {"assets/ex1/coloring/jean.col", jean, 10}
gen_test! {"assets/ex1/coloring/anna.col", anna, 11}
gen_test! {"assets/ex1/coloring/david.col", david, 11}
gen_test! {"assets/ex1/coloring/huck.col", huck, 11}
gen_test! {"assets/ex1/coloring/queen11_11.col", queen11_11, 11}
gen_test! {"assets/ex1/coloring/queen13_13.col", queen13_13, 13}
gen_test! {"assets/ex1/coloring/homer.col", homer, 13}
gen_test! {"assets/ex1/coloring/le450_15b.col", le450_15b, 15}
gen_test! {"assets/ex1/coloring/le450_15c.col", le450_15c, 15}
gen_test! {"assets/ex1/coloring/le450_15d.col", le450_15d, 15}
gen_test! {"assets/ex1/coloring/miles500.col", miles500, 20}
gen_test! {"assets/ex1/coloring/le450_25a.col", le450_25a, 25}
gen_test! {"assets/ex1/coloring/le450_25b.col", le450_25b, 25}
gen_test! {"assets/ex1/coloring/le450_25c.col", le450_25c, 25}
gen_test! {"assets/ex1/coloring/le450_25d.col", le450_25d, 25}
gen_test! {"assets/ex1/coloring/zeroin.i.2.col", zeroin_i_2, 30}
gen_test! {"assets/ex1/coloring/zeroin.i.3.col", zeroin_i_3, 30}
gen_test! {"assets/ex1/coloring/miles750.col", miles750, 31}
gen_test! {"assets/ex1/coloring/mulsol.i.2.col", mulsol_i_2, 31}
gen_test! {"assets/ex1/coloring/mulsol.i.3.col", mulsol_i_3, 31}
gen_test! {"assets/ex1/coloring/mulsol.i.4.col", mulsol_i_4, 31}
gen_test! {"assets/ex1/coloring/mulsol.i.5.col", mulsol_i_5, 31}
gen_test! {"assets/ex1/coloring/inithx.i.2.col", inithx_i_2, 31}
gen_test! {"assets/ex1/coloring/inithx.i.3.col", inithx_i_3, 31}
gen_test! {"assets/ex1/coloring/miles1000.col", miles1000, 42}
gen_test! {"assets/ex1/coloring/mulsol.i.1.col", mulsol_i_1, 49}
gen_test! {"assets/ex1/coloring/zeroin.i.1.col", zeroin_i_1, 49}
gen_test! {"assets/ex1/coloring/inithx.i.1.col", inithx_i_1, 54}
gen_test! {"assets/ex1/coloring/miles1500.col", miles1500, 73}
