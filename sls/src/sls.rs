use std::time::SystemTime;
use solver::timer::Timer;

use crate::assignment::Assignment;
use crate::cnf::Cnf;
use crate::context::Context;

#[derive(Debug, Copy, Clone)]
pub enum Strategy {
    GSat,
    WalkSat,
}

pub fn solve(cnf: Cnf, strategy: Strategy, timer: Option<Timer>) -> Option<Assignment> {
    let max_tries = 1000000;
    let max_flips = 10 * cnf.max_var_id;

    let mut context = Context::new(Assignment::new_random(cnf.max_var_id as usize), cnf);

    for _ in 0..max_tries {
        for _ in 0..max_flips {
            let flip_var = match strategy {
                Strategy::GSat => context.min_break_count(),
                Strategy::WalkSat => context.max_diffscore(),
            };
            context.flip(flip_var);

            if context.is_sat() {
                return Some(context.into());
            }

            if let Some(timer) = timer {
                if timer.has_finished() {
                    return None;
                }
            }
        }
        context.randomize_assignment();
    }

    None
}
