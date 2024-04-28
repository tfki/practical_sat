use std::ffi::{c_uint, c_void};
use std::ops::RangeFrom;
use std::os::raw::c_int;

use crate::cnf::literal::Lit;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SolveResult {
    Sat,
    Unsat,
    Interrupted,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LitValue {
    True,
    False,
    DontCare,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SolverState {
    Sat,
    Unsat,
    Input,
}

extern "C" fn terminate_fn_wrapper(state: *mut c_void) -> c_int
{
    let f: &mut Box<dyn FnMut() -> bool> = unsafe {
        &mut *(state as *mut Box<dyn FnMut() -> bool>)
    };
    match f() {
        true => 1,
        false => 0,
    }
}


pub struct Solver {
    solver_ptr: *mut c_void,
    state: SolverState,
    terminate_fn: Option<Box<dyn FnMut() -> bool>>,
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { ipasir_sys::ipasir_release(self.solver_ptr) }
    }
}

impl Solver {
    pub fn new() -> Self {
        Self {
            state: SolverState::Input,
            solver_ptr: unsafe {
                ipasir_sys::ipasir_init()
            },
            terminate_fn: None,
        }
    }

    pub fn add_literal(&mut self, lit: Lit) {
        unsafe {
            ipasir_sys::ipasir_add(self.solver_ptr, lit.into())
        }
        self.state = SolverState::Input;
    }

    pub fn add_clause(&mut self, clause: &[Lit]) {
        for lit in clause {
            self.add_literal(*lit);
        }
        self.add_literal(Lit { id: 0, negated: false });
    }

    pub fn assume(&mut self, lit: Lit) {
        unsafe { ipasir_sys::ipasir_assume(self.solver_ptr, lit.into()) }
    }

    pub fn solve(&mut self) -> SolveResult {
        match unsafe { ipasir_sys::ipasir_solve(self.solver_ptr) } {
            0 => {
                self.state = SolverState::Input;
                SolveResult::Interrupted
            }
            10 => {
                self.state = SolverState::Sat;
                SolveResult::Sat
            }
            20 => {
                self.state = SolverState::Unsat;
                SolveResult::Unsat
            }
            _ => unreachable!(),
        }
    }

    pub fn val(&mut self, lit: Lit) -> LitValue {
        assert!(matches!(self.state, SolverState::Sat));

        let val = unsafe {
            ipasir_sys::ipasir_val(self.solver_ptr, lit.into())
        };
        
        if val == 0 {
            LitValue::DontCare
        } else if val < 0 {
            LitValue::False
        } else {
            LitValue::True
        }
    }

    pub fn set_terminate<F>(&mut self, cb: F)
        where
            F: 'static + FnMut() -> bool,
    {
        self.terminate_fn = Some(Box::new(cb));
        unsafe {
            ipasir_sys::ipasir_set_terminate(
                self.solver_ptr,
                self.terminate_fn.as_mut().unwrap() as *mut _ as *mut std::os::raw::c_void,
                Some(terminate_fn_wrapper),
            )
        }
    }

    pub fn at_least_one(&mut self, lits: &[Lit]) {
        self.add_clause(lits);
    }

    pub fn at_most_one_pairwise(&mut self, lits: &[Lit]) {
        for i in 0..lits.len() {
            for j in (i + 1)..lits.len() {
                self.add_clause(&[-lits[i], -lits[j]]);
            }
        }
    }

    pub fn exactly_k_seq_counter(&mut self, lits: &[Lit], k: u32, allocator: &mut RangeFrom<c_uint>) {
        if k > lits.len() as u32 {
            self.add_clause(&[]);
        }

        let last_layer_outputs = self.seq_counter(lits, allocator);

        if k > 0 {
            if let Some(x) = last_layer_outputs.get((k - 1) as usize) {
                self.add_clause(&[*x]);
            }
        }
        if let Some(x) = last_layer_outputs.get(k as usize) {
            self.add_clause(&[-*x]);
        }
    }

    pub fn at_least_k_seq_counter(&mut self, lits: &[Lit], k: u32, allocator: &mut RangeFrom<c_uint>) {
        if k > lits.len() as u32 {
            self.add_clause(&[]);
        }

        let last_layer_outputs = self.seq_counter(lits, allocator);

        if k > 0 {
            if let Some(x) = last_layer_outputs.get((k - 1) as usize) {
                self.add_clause(&[*x]);
            }
        }
    }

    pub fn at_most_k_seq_counter(&mut self, lits: &[Lit], k: u32, allocator: &mut RangeFrom<c_uint>) {
        if k > lits.len() as u32 {
            self.add_clause(&[]);
        }

        let last_layer_outputs = self.seq_counter(lits, allocator);

        if let Some(x) = last_layer_outputs.get(k as usize) {
            self.add_clause(&[-*x]);
        }
    }

    fn seq_counter(&mut self, lits: &[Lit], allocator: &mut RangeFrom<c_uint>) -> Vec<Lit> {
        let mut prev_layer_outputs = vec![lits[0]];

        for lit in &lits[1..] {
            let mut layer_outputs = vec![];
            for _ in 0..=prev_layer_outputs.len() {
                layer_outputs.push(Lit::new(allocator.next().unwrap()));
            };

            self.add_clause(&[layer_outputs[0], -*lit]);
            self.add_clause(&[layer_outputs[0], -prev_layer_outputs[0]]);
            self.add_clause(&[-layer_outputs[0], prev_layer_outputs[0], *lit]);

            for (i, layer_output) in layer_outputs.iter().enumerate() {
                if i == 0 || i == layer_outputs.len() - 1 { continue; }

                self.add_clause(&[-*layer_output, prev_layer_outputs[i], *lit]);
                self.add_clause(&[-*layer_output, prev_layer_outputs[i], prev_layer_outputs[i - 1]]);

                self.add_clause(&[*layer_output, -prev_layer_outputs[i], -prev_layer_outputs[i - 1]]);
                self.add_clause(&[*layer_output, -prev_layer_outputs[i], -*lit]);
                self.add_clause(&[*layer_output, -prev_layer_outputs[i - 1], -*lit]);
            }

            self.add_clause(&[-*layer_outputs.last().unwrap(), *lit]);
            self.add_clause(&[-*layer_outputs.last().unwrap(), *prev_layer_outputs.last().unwrap()]);

            self.add_clause(&[*layer_outputs.last().unwrap(), -*lit, -*prev_layer_outputs.last().unwrap()]);
            prev_layer_outputs = layer_outputs;
        }

        prev_layer_outputs
    }
}
