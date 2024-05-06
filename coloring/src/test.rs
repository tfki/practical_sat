macro_rules! gen_test {
    ($path:expr, $name:ident, $k:expr, $f:expr) => {

        #[test]
        fn $name() {
            use std::path::Path;
            use crate::FindKResult;
            use crate::graph::Graph;
            use std::time::Duration;
            use solver::timer::Timer;

            let path_string = String::from($path);
            let path = Path::new(&path_string);
            let graph = Graph::parse_dimacs(path);

            let result = $f(graph, Timer::new(Duration::from_secs(60)));
            println!("{result:?}");

            assert!(matches!(result, FindKResult::Found($k)));
        }
    };
}

mod one_hot_incremental {
    gen_test! {"assets/myciel3.col", myciel3, 4, crate::one_hot_incremental::find_k}
    gen_test! {"assets/myciel4.col", myciel4, 5, crate::one_hot_incremental::find_k}
    gen_test! {"assets/le450_5a.col", le450_5a, 5, crate::one_hot_incremental::find_k}
    gen_test! {"assets/le450_5b.col", le450_5b, 5, crate::one_hot_incremental::find_k}
    gen_test! {"assets/le450_5c.col", le450_5c, 5, crate::one_hot_incremental::find_k}
    gen_test! {"assets/le450_5d.col", le450_5d, 5, crate::one_hot_incremental::find_k}
    gen_test! {"assets/myciel5.col", myciel5, 6, crate::one_hot_incremental::find_k}
    gen_test! {"assets/myciel6.col", myciel6, 7, crate::one_hot_incremental::find_k}
    gen_test! {"assets/myciel7.col", myciel7, 8, crate::one_hot_incremental::find_k}
    gen_test! {"assets/miles250.col", miles250, 8, crate::one_hot_incremental::find_k}
    gen_test! {"assets/jean.col", jean, 10, crate::one_hot_incremental::find_k}
    gen_test! {"assets/anna.col", anna, 11, crate::one_hot_incremental::find_k}
    gen_test! {"assets/david.col", david, 11, crate::one_hot_incremental::find_k}
    gen_test! {"assets/huck.col", huck, 11, crate::one_hot_incremental::find_k}
    gen_test! {"assets/queen11_11.col", queen11_11, 11, crate::one_hot_incremental::find_k}
    gen_test! {"assets/queen13_13.col", queen13_13, 13, crate::one_hot_incremental::find_k}
    gen_test! {"assets/homer.col", homer, 13, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_15b.col", le450_15b, 15, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_15c.col", le450_15c, 15, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_15d.col", le450_15d, 15, crate::one_hot_incremental::find_k}
// gen_test! {"assets/miles500.col", miles500, 20, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_25a.col", le450_25a, 25, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_25b.col", le450_25b, 25, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_25c.col", le450_25c, 25, crate::one_hot_incremental::find_k}
// gen_test! {"assets/le450_25d.col", le450_25d, 25, crate::one_hot_incremental::find_k}
// gen_test! {"assets/zeroin.i.2.col", zeroin_i_2, 30, crate::one_hot_incremental::find_k}
// gen_test! {"assets/zeroin.i.3.col", zeroin_i_3, 30, crate::one_hot_incremental::find_k}
// gen_test! {"assets/miles750.col", miles750, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/mulsol.i.2.col", mulsol_i_2, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/mulsol.i.3.col", mulsol_i_3, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/mulsol.i.4.col", mulsol_i_4, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/mulsol.i.5.col", mulsol_i_5, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/inithx.i.2.col", inithx_i_2, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/inithx.i.3.col", inithx_i_3, 31, crate::one_hot_incremental::find_k}
// gen_test! {"assets/miles1000.col", miles1000, 42, crate::one_hot_incremental::find_k}
// gen_test! {"assets/mulsol.i.1.col", mulsol_i_1, 49, crate::one_hot_incremental::find_k}
// gen_test! {"assets/zeroin.i.1.col", zeroin_i_1, 49, crate::one_hot_incremental::find_k}
// gen_test! {"assets/inithx.i.1.col", inithx_i_1, 54, crate::one_hot_incremental::find_k}
// gen_test! {"assets/miles1500.col", miles1500, 73, crate::one_hot_incremental::find_k}
}

mod bitvec_incremental {
    gen_test! {"assets/myciel3.col", myciel3, 4, crate::bitvec_incremental::find_k}
    gen_test! {"assets/myciel4.col", myciel4, 5, crate::bitvec_incremental::find_k}
    gen_test! {"assets/le450_5a.col", le450_5a, 5, crate::bitvec_incremental::find_k}
    gen_test! {"assets/le450_5b.col", le450_5b, 5, crate::bitvec_incremental::find_k}
    gen_test! {"assets/le450_5c.col", le450_5c, 5, crate::bitvec_incremental::find_k}
    gen_test! {"assets/le450_5d.col", le450_5d, 5, crate::bitvec_incremental::find_k}
    gen_test! {"assets/myciel5.col", myciel5, 6, crate::bitvec_incremental::find_k}
    gen_test! {"assets/myciel6.col", myciel6, 7, crate::bitvec_incremental::find_k}
    gen_test! {"assets/myciel7.col", myciel7, 8, crate::bitvec_incremental::find_k}
    gen_test! {"assets/miles250.col", miles250, 8, crate::bitvec_incremental::find_k}
    gen_test! {"assets/jean.col", jean, 10, crate::bitvec_incremental::find_k}
    gen_test! {"assets/anna.col", anna, 11, crate::bitvec_incremental::find_k}
    gen_test! {"assets/david.col", david, 11, crate::bitvec_incremental::find_k}
    gen_test! {"assets/huck.col", huck, 11, crate::bitvec_incremental::find_k}
    gen_test! {"assets/queen11_11.col", queen11_11, 11, crate::bitvec_incremental::find_k}
    gen_test! {"assets/queen13_13.col", queen13_13, 13, crate::bitvec_incremental::find_k}
    gen_test! {"assets/homer.col", homer, 13, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_15b.col", le450_15b, 15, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_15c.col", le450_15c, 15, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_15d.col", le450_15d, 15, crate::bitvec_incremental::find_k}
// gen_test! {"assets/miles500.col", miles500, 20, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_25a.col", le450_25a, 25, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_25b.col", le450_25b, 25, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_25c.col", le450_25c, 25, crate::bitvec_incremental::find_k}
// gen_test! {"assets/le450_25d.col", le450_25d, 25, crate::bitvec_incremental::find_k}
// gen_test! {"assets/zeroin.i.2.col", zeroin_i_2, 30, crate::bitvec_incremental::find_k}
// gen_test! {"assets/zeroin.i.3.col", zeroin_i_3, 30, crate::bitvec_incremental::find_k}
// gen_test! {"assets/miles750.col", miles750, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/mulsol.i.2.col", mulsol_i_2, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/mulsol.i.3.col", mulsol_i_3, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/mulsol.i.4.col", mulsol_i_4, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/mulsol.i.5.col", mulsol_i_5, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/inithx.i.2.col", inithx_i_2, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/inithx.i.3.col", inithx_i_3, 31, crate::bitvec_incremental::find_k}
// gen_test! {"assets/miles1000.col", miles1000, 42, crate::bitvec_incremental::find_k}
// gen_test! {"assets/mulsol.i.1.col", mulsol_i_1, 49, crate::bitvec_incremental::find_k}
// gen_test! {"assets/zeroin.i.1.col", zeroin_i_1, 49, crate::bitvec_incremental::find_k}
// gen_test! {"assets/inithx.i.1.col", inithx_i_1, 54, crate::bitvec_incremental::find_k}
// gen_test! {"assets/miles1500.col", miles1500, 73, crate::bitvec_incremental::find_k}
}

mod hybrid {
    gen_test! {"assets/myciel3.col", myciel3, 4, crate::hybrid::find_k}
    gen_test! {"assets/myciel4.col", myciel4, 5, crate::hybrid::find_k}
    gen_test! {"assets/le450_5a.col", le450_5a, 5, crate::hybrid::find_k}
    gen_test! {"assets/le450_5b.col", le450_5b, 5, crate::hybrid::find_k}
    gen_test! {"assets/le450_5c.col", le450_5c, 5, crate::hybrid::find_k}
    gen_test! {"assets/le450_5d.col", le450_5d, 5, crate::hybrid::find_k}
    gen_test! {"assets/myciel5.col", myciel5, 6, crate::hybrid::find_k}
    gen_test! {"assets/myciel6.col", myciel6, 7, crate::hybrid::find_k}
    gen_test! {"assets/myciel7.col", myciel7, 8, crate::hybrid::find_k}
    gen_test! {"assets/miles250.col", miles250, 8, crate::hybrid::find_k}
    gen_test! {"assets/jean.col", jean, 10, crate::hybrid::find_k}
    gen_test! {"assets/anna.col", anna, 11, crate::hybrid::find_k}
    gen_test! {"assets/david.col", david, 11, crate::hybrid::find_k}
    gen_test! {"assets/huck.col", huck, 11, crate::hybrid::find_k}
    gen_test! {"assets/queen11_11.col", queen11_11, 11, crate::hybrid::find_k}
    gen_test! {"assets/queen13_13.col", queen13_13, 13, crate::hybrid::find_k}
    gen_test! {"assets/homer.col", homer, 13, crate::hybrid::find_k}
// gen_test! {"assets/le450_15b.col", le450_15b, 15, crate::hybrid::find_k}
// gen_test! {"assets/le450_15c.col", le450_15c, 15, crate::hybrid::find_k}
// gen_test! {"assets/le450_15d.col", le450_15d, 15, crate::hybrid::find_k}
// gen_test! {"assets/miles500.col", miles500, 20, crate::hybrid::find_k}
// gen_test! {"assets/le450_25a.col", le450_25a, 25, crate::hybrid::find_k}
// gen_test! {"assets/le450_25b.col", le450_25b, 25, crate::hybrid::find_k}
// gen_test! {"assets/le450_25c.col", le450_25c, 25, crate::hybrid::find_k}
// gen_test! {"assets/le450_25d.col", le450_25d, 25, crate::hybrid::find_k}
// gen_test! {"assets/zeroin.i.2.col", zeroin_i_2, 30, crate::hybrid::find_k}
// gen_test! {"assets/zeroin.i.3.col", zeroin_i_3, 30, crate::hybrid::find_k}
// gen_test! {"assets/miles750.col", miles750, 31, crate::hybrid::find_k}
// gen_test! {"assets/mulsol.i.2.col", mulsol_i_2, 31, crate::hybrid::find_k}
// gen_test! {"assets/mulsol.i.3.col", mulsol_i_3, 31, crate::hybrid::find_k}
// gen_test! {"assets/mulsol.i.4.col", mulsol_i_4, 31, crate::hybrid::find_k}
// gen_test! {"assets/mulsol.i.5.col", mulsol_i_5, 31, crate::hybrid::find_k}
// gen_test! {"assets/inithx.i.2.col", inithx_i_2, 31, crate::hybrid::find_k}
// gen_test! {"assets/inithx.i.3.col", inithx_i_3, 31, crate::hybrid::find_k}
// gen_test! {"assets/miles1000.col", miles1000, 42, crate::hybrid::find_k}
// gen_test! {"assets/mulsol.i.1.col", mulsol_i_1, 49, crate::hybrid::find_k}
// gen_test! {"assets/zeroin.i.1.col", zeroin_i_1, 49, crate::hybrid::find_k}
// gen_test! {"assets/inithx.i.1.col", inithx_i_1, 54, crate::hybrid::find_k}
// gen_test! {"assets/miles1500.col", miles1500, 73, crate::hybrid::find_k}
}
