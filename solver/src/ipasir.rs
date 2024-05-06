use std::cmp::Ordering;
use std::ffi::{c_void};
use std::os::raw::c_int;
use std::time::Duration;
use crate::literal::Lit;
use crate::{LitValue, SolveResult, SolverImpl, SolveWithTimeoutResult};

use crate::timer::Timer;

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
    fn set_terminate<F>(&mut self, cb: F)
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
}

impl SolverImpl for Solver {
    fn new() -> Self {
        Self {
            state: SolverState::Input,
            solver_ptr: unsafe {
                ipasir_sys::ipasir_init()
            },
            terminate_fn: None,
        }
    }

    fn add_literal(&mut self, lit: Lit) {
        unsafe {
            ipasir_sys::ipasir_add(self.solver_ptr, lit.into())
        }
        self.state = SolverState::Input;
    }

    fn assume(&mut self, lit: Lit) {
        unsafe { ipasir_sys::ipasir_assume(self.solver_ptr, lit.into()) }
    }

    fn solve(&mut self) -> SolveResult {
        match unsafe { ipasir_sys::ipasir_solve(self.solver_ptr) } {
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

    fn solve_with_timeout(&mut self, timeout: Duration) -> SolveWithTimeoutResult {
        let timer = Timer::new(timeout);
        self.set_terminate(move || timer.has_finished());

        match unsafe { ipasir_sys::ipasir_solve(self.solver_ptr) } {
            0 => {
                self.state = SolverState::Input;
                SolveWithTimeoutResult::TimeoutReached
            }
            10 => {
                self.state = SolverState::Sat;
                SolveWithTimeoutResult::Sat
            }
            20 => {
                self.state = SolverState::Unsat;
                SolveWithTimeoutResult::Unsat
            }
            _ => unreachable!(),
        }
    }

    fn val(&mut self, lit: Lit) -> LitValue {
        assert!(matches!(self.state, SolverState::Sat));

        let val = unsafe {
            ipasir_sys::ipasir_val(self.solver_ptr, lit.into())
        };

        match val.cmp(&0) {
            Ordering::Less => LitValue::False,
            Ordering::Equal => LitValue::DontCare,
            Ordering::Greater => LitValue::True,
        }
    }
}
