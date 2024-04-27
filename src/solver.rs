use std::ffi::c_void;
use std::mem;
use std::os::raw::c_int;

use crate::cnf::literal::{Literal, Variable};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SolveResult {
    Sat,
    Unsat,
    Interrupted,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VariableValue {
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

    pub fn add_literal(&mut self, lit: Literal) {
        unsafe {
            ipasir_sys::ipasir_add(self.solver_ptr, lit.into())
        }
        self.state = SolverState::Input;
    }

    pub fn add_clause(&mut self, clause: &[Literal]) {
        for lit in clause {
            self.add_literal(*lit);
        }
        self.add_literal(Literal { var: Variable { id: 0 }, negated: false });
    }

    pub fn assume_literal(&mut self, lit: Literal) {
        unsafe { ipasir_sys::ipasir_assume(self.solver_ptr, lit.into()) }
    }

    pub fn assume_clause(&mut self, clause: &[Literal]) {
        for lit in clause {
            self.assume_literal(*lit);
        }
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

    pub fn val(&mut self, var: Variable) -> VariableValue {
        assert!(matches!(self.state, SolverState::Sat));

        let val = unsafe { ipasir_sys::ipasir_val(self.solver_ptr, var.id as i32) };

        if val == var.id as i32 {
            VariableValue::True
        } else if -val == var.id as i32 {
            VariableValue::False
        } else {
            VariableValue::DontCare
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

    pub fn at_most_one_pairwise(&mut self, lits: &[Literal]) {
        for i in 0..lits.len() {
            for j in (i + 1)..lits.len() {
                self.add_clause(&[-lits[i], -lits[j]]);
            }
        }
    }

    pub fn at_most_k_sequential_counter(&mut self, _k: u32, _lits: &[Literal]) {
        todo!()
    }
}
