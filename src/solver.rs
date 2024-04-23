use std::ffi::{c_int, c_void};
use crate::cnf::literal::{Literal, Variable};

pub enum SolveResult {
    Sat,
    Unsat,
    Interrupted,
}

pub enum VariableValue {
    True,
    False, DontCare
}

enum SolverState {
    Sat, Unsat, Input
}

pub struct Solver {
    solver_ptr: *mut c_void,
    state: SolverState,
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
            }
        }
    }

    pub fn add_literal(&mut self, lit: Literal) {
        unsafe {
            ipasir_sys::ipasir_add(self.solver_ptr, lit.into())
        }
        self.state = SolverState::Input;
    }

    pub fn solve(&mut self) -> SolveResult {
        match unsafe { ipasir_sys::ipasir_solve(self.solver_ptr) } {
            20 => SolveResult::Sat,
            10 => SolveResult::Unsat,
            0 => SolveResult::Interrupted,
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
}
