use solver::{ipasir, LitValue, Solver, SolveResult, SolverImpl};
use solver::literal::Lit;

fn main() {
    let mut solver = Solver::<ipasir::Solver>::new();

    let bits_a = (0..64).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
    let bits_b = (0..64).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
    // let bits_o = (0..8).map(|_| solver.new_lit()).collect::<Vec<Lit>>();

    let always_false = solver.new_lit();
    solver.add_clause([-always_false]);

    let mut intermediates = vec![];
    for bit_b in bits_b.iter() {
        let intermediate = bits_a.iter().map(|_| solver.new_lit()).collect::<Vec<Lit>>();
        encode_if(&bits_a, &intermediate, *bit_b, &mut solver);
        intermediates.push(intermediate);
    }

    let mut last_accumulator = (0..64).map(|_| solver.new_lit()).collect::<Vec<Lit>>();
    last_accumulator.iter().for_each(|bit| solver.add_clause([-*bit]));

    for i in 0..(intermediates.len() - 1) {
        let new_accumulator = (0..=last_accumulator.len()).map(|_| solver.new_lit()).collect::<Vec<Lit>>();

        while intermediates[i].len() < last_accumulator.len() {
            intermediates[i] = prepend(&intermediates[i], always_false);
        }

        encode_full_adder(&last_accumulator, &intermediates[i], &new_accumulator, &mut solver);

        last_accumulator = new_accumulator;
    }
    
    for a in 0..1000 {
        for b in 0..1000 {
            for (i, bit) in bits_a.iter().enumerate() {
                let value = a & (1 << i) != 0;
                if value {
                    solver.assume(*bit);
                } else {
                    solver.assume(-*bit);
                }
            }
            for (i, bit) in bits_b.iter().enumerate() {
                let value = b & (1 << i) != 0;
                if value {
                    solver.assume(*bit);
                } else {
                    solver.assume(-*bit);
                }
            }
            
            match solver.solve() {
                SolveResult::Sat => {
                    let mut output = 0_u64;
                    for (i, bit) in last_accumulator.iter().enumerate().take(64) {
                        match solver.val(*bit) {
                            LitValue::True => output |= 1 << i,
                            LitValue::False => {}
                            LitValue::DontCare => {}
                        }
                    }
                    
                    println!("{a} * {b} = {output}");
                    assert_eq!(a * b, output);
                }
                SolveResult::Unsat => panic!(),
            }
        }
    }
}

fn prepend(a: &Vec<Lit>, lit: Lit) -> Vec<Lit> {
    let mut result = vec![];

    result.push(lit);
    result.append(&mut a.clone());

    result
}

fn append(a: &Vec<Lit>, lit: Lit) -> Vec<Lit> {
    let mut result = vec![];

    result.append(&mut a.clone());
    result.push(lit);

    result
}

fn encode_if<I: SolverImpl>(a: &[Lit], output: &[Lit], switch: Lit, solver: &mut Solver<I>) {
    assert_eq!(a.len(), output.len());

    for i in 0..a.len() {
        solver.add_clause([-a[i], -switch, output[i]]);
        solver.add_clause([a[i], -output[i]]);
        solver.add_clause([switch, -output[i]]);
    }
}

fn encode_full_adder<I: SolverImpl>(a: &[Lit], b: &[Lit], output: &[Lit], solver: &mut Solver<I>) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len() + 1, output.len());

    let carries = (0..a.len()).map(|_| solver.new_lit()).collect::<Vec<Lit>>();

    solver.add_clause([-a[0], -b[0], carries[0]]);
    solver.add_clause([-carries[0], a[0]]);
    solver.add_clause([-carries[0], b[0]]);

    solver.add_clause([-a[0], b[0], output[0]]);
    solver.add_clause([a[0], -b[0], output[0]]);
    solver.add_clause([a[0], b[0], -output[0]]);
    solver.add_clause([-a[0], -b[0], -output[0]]);

    for i in 1..carries.len() {
        let a = a[i];
        let b = b[i];
        let c_last = carries[i - 1];
        {
            let o = output[i];

            //------------------------------ output
            let c1 = solver.new_lit();
            solver.add_clause([-c1, a]);
            solver.add_clause([-c1, b]);
            solver.add_clause([-c1, c_last]);
            solver.add_clause([-a, -b, -c_last, c1]);

            let c2 = solver.new_lit();
            solver.add_clause([-c2, -a]);
            solver.add_clause([-c2, -b]);
            solver.add_clause([-c2, c_last]);
            solver.add_clause([a, b, -c_last, c2]);

            let c3 = solver.new_lit();
            solver.add_clause([-c3, -a]);
            solver.add_clause([-c3, b]);
            solver.add_clause([-c3, -c_last]);
            solver.add_clause([a, -b, c_last, c3]);

            let c4 = solver.new_lit();
            solver.add_clause([-c4, a]);
            solver.add_clause([-c4, -b]);
            solver.add_clause([-c4, -c_last]);
            solver.add_clause([-a, b, c_last, c4]);

            solver.add_clause([-o, c1, c2, c3, c4]);
            solver.add_clause([-c1, o]);
            solver.add_clause([-c2, o]);
            solver.add_clause([-c3, o]);
            solver.add_clause([-c4, o]);
        }
        {
            let c_next = carries[i];

            //------------------------------ carry
            let d1 = solver.new_lit();
            solver.add_clause([-d1, c_last]);
            solver.add_clause([-d1, a]);
            solver.add_clause([d1, -a, -c_last]);

            let d2 = solver.new_lit();
            solver.add_clause([-d2, c_last]);
            solver.add_clause([-d2, b]);
            solver.add_clause([d2, -b, -c_last]);

            let d3 = solver.new_lit();
            solver.add_clause([-d3, a]);
            solver.add_clause([-d3, b]);
            solver.add_clause([d3, -a, -b]);


            solver.add_clause([-c_next, d1, d2, d3]);
            solver.add_clause([-d1, c_next]);
            solver.add_clause([-d2, c_next]);
            solver.add_clause([-d3, c_next]);
        }
    }
}
