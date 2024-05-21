use std::cmp::Ordering;
use std::vec;

use rand::{Rng, thread_rng};
use solver::literal::Lit;
use solver::variable::Var;

use crate::assignment::Assignment;
use crate::cnf::Cnf;

#[derive(Debug, Clone)]
pub struct CnfWithMeta {
    pub clauses: Vec<ClauseWithMeta>,
    pub max_var_id: u32,
}

#[derive(Debug, Clone)]
pub struct ClauseWithMeta {
    clause: Vec<Lit>,
    state: ClauseState,
}

#[derive(Debug)]
pub struct Context {
    pub cnf: CnfWithMeta,
    assignment: Assignment,
    pub var_to_clause_index: Vec<Vec<u32>>,
    pub make_counts: Vec<u32>,
    pub break_counts: Vec<u32>,
    pub num_unsat_clauses: u32,

    make_break_buffer: Vec<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct ClauseState {
    impact_of_flip: ImpactOfFlip,
    sat: bool,
}

#[derive(Debug, Clone, Default)]
enum ImpactOfFlip {
    #[default]
    Unknown,
    MakeableBy(Vec<Var>),
    BreakableBy(Var),
    None,
}

impl Context {
    pub fn new(assignment: Assignment, cnf: Cnf) -> Self {
        let mut cnf_with_meta = CnfWithMeta { clauses: vec![], max_var_id: cnf.max_var_id };
        let mut make_counts = vec![0; cnf.max_var_id as usize + 1];
        let mut break_counts = vec![0; cnf.max_var_id as usize + 1];
        let mut var_to_clause_index = vec![vec![]; cnf.max_var_id as usize + 1];
        let mut unsat_clause_counter = 0;

        for (i, clause) in cnf.clauses.into_iter().enumerate() {
            for lit in clause.iter() {
                var_to_clause_index[lit.var.id as usize].push(i as u32);
            }

            let clause_state = get_clause_state(&assignment, &clause);
            if !clause_state.sat {
                unsat_clause_counter += 1;
            }
            match &clause_state.impact_of_flip {
                ImpactOfFlip::Unknown => panic!("empty clause in input?"),
                ImpactOfFlip::MakeableBy(vars) => {
                    for var in vars {
                        make_counts[var.id as usize] += 1;
                    }
                }
                ImpactOfFlip::BreakableBy(var) => break_counts[var.id as usize] += 1,
                ImpactOfFlip::None => {}
            }
            cnf_with_meta.clauses.push(ClauseWithMeta { clause, state: clause_state });
        }

        Context {
            cnf: cnf_with_meta,
            assignment,
            var_to_clause_index,
            make_counts,
            break_counts,
            num_unsat_clauses: unsat_clause_counter,

            make_break_buffer: vec![],
        }
    }

    pub fn reinit(&mut self) {
        self.num_unsat_clauses = 0;
        self.make_counts.fill(0);
        self.break_counts.fill(0);
        self.var_to_clause_index.iter_mut().for_each(|v| v.clear());

        for (i, clause_w_meta) in self.cnf.clauses.iter_mut().enumerate() {
            for lit in clause_w_meta.clause.iter() {
                self.var_to_clause_index[lit.var.id as usize].push(i as u32);
            }

            let new_clause_state = get_clause_state(&self.assignment, &clause_w_meta.clause);
            if !new_clause_state.sat {
                self.num_unsat_clauses += 1;
            }
            match &new_clause_state.impact_of_flip {
                ImpactOfFlip::Unknown => panic!("empty clause in input?"),
                ImpactOfFlip::MakeableBy(vars) => {
                    for var in vars {
                        self.make_counts[var.id as usize] += 1;
                    }
                }
                ImpactOfFlip::BreakableBy(var) => self.break_counts[var.id as usize] += 1,
                ImpactOfFlip::None => {}
            }
            clause_w_meta.state = new_clause_state;
        }
    }

    pub fn assignment(&self) -> &Assignment {
        &self.assignment
    }

    pub fn is_sat(&self) -> bool {
        self.num_unsat_clauses == 0
    }

    pub fn flip(&mut self, flip_var: Var) {
        self.assignment[flip_var] = !self.assignment[flip_var];
        for clause_i in &self.var_to_clause_index[flip_var.id as usize] {
            let new_state = get_clause_state(&self.assignment, &self.cnf.clauses[*clause_i as usize].clause);
            let prev_state = &self.cnf.clauses[*clause_i as usize].state;

            match (prev_state.sat, new_state.sat) {
                (true, false) => self.num_unsat_clauses += 1,
                (false, true) => self.num_unsat_clauses -= 1,
                (_, _) => {}
            }

            match (&prev_state.impact_of_flip, &new_state.impact_of_flip) {
                (ImpactOfFlip::None, ImpactOfFlip::MakeableBy(vars)) => {
                    for var in vars {
                        self.make_counts[var.id as usize] += 1;
                    }
                }
                (ImpactOfFlip::None, ImpactOfFlip::BreakableBy(var)) => {
                    self.break_counts[var.id as usize] += 1;
                }
                (ImpactOfFlip::BreakableBy(var), ImpactOfFlip::None) => {
                    self.break_counts[var.id as usize] -= 1;
                }
                (ImpactOfFlip::BreakableBy(break_var), ImpactOfFlip::MakeableBy(make_vars)) => {
                    self.break_counts[break_var.id as usize] -= 1;
                    for var in make_vars {
                        self.make_counts[var.id as usize] += 1;
                    }
                }
                (ImpactOfFlip::MakeableBy(_), ImpactOfFlip::None) => {
                    unreachable!()
                }
                (ImpactOfFlip::MakeableBy(make_vars), ImpactOfFlip::BreakableBy(break_var)) => {
                    self.break_counts[break_var.id as usize] += 1;
                    for var in make_vars {
                        self.make_counts[var.id as usize] -= 1;
                    }
                }
                (_, _) => {}
            }

            self.cnf.clauses[*clause_i as usize].state = new_state;
        }
    }

    pub fn min_break_count(&mut self) -> Var {
        let mut min_break_count = u32::MAX;
        self.make_break_buffer.clear();

        for (i, candidate) in self.break_counts.iter().enumerate().skip(1) {
            match candidate.cmp(&min_break_count) {
                Ordering::Less => {
                    min_break_count = *candidate;

                    self.make_break_buffer.clear();
                    self.make_break_buffer.push(i as u32);
                }
                Ordering::Equal => {
                    self.make_break_buffer.push(i as u32);
                }
                _ => {}
            }
        }

        Var { id: self.make_break_buffer[thread_rng().gen_range(0..self.make_break_buffer.len())] }
    }

    pub fn max_diffscore(&mut self) -> Var {
        let mut max_diffscore = i32::MAX;
        self.make_break_buffer.clear();

        debug_assert_eq!(self.make_counts.len(), self.break_counts.len());
        for i in 1..self.break_counts.len() {
            let break_count = self.break_counts[i];
            let make_count = self.make_counts[i];
            
            let diffscore = break_count as i32 - make_count as i32;
            
            match diffscore.cmp(&max_diffscore) {
                Ordering::Less => {
                    max_diffscore = diffscore;

                    self.make_break_buffer.clear();
                    self.make_break_buffer.push(i as u32);
                }
                Ordering::Equal => {
                    self.make_break_buffer.push(i as u32);
                }
                _ => {}
            }
        }

        Var { id: self.make_break_buffer[thread_rng().gen_range(0..self.make_break_buffer.len())] }
    }

    pub fn make_count_of_flip(&self, var: Var) -> u32 {
        self.make_counts[var.id as usize]
    }

    pub fn break_count_of_flip(&self, var: Var) -> u32 {
        self.break_counts[var.id as usize]
    }

    pub fn randomize_assignment(&mut self) {
        self.assignment.randomize();
        self.reinit();
    }
}

impl From<Context> for Assignment {
    fn from(value: Context) -> Self {
        value.assignment
    }
}

fn get_clause_state(assignment: &Assignment, clause: &[Lit]) -> ClauseState {
    let mut impact_of_flip = ImpactOfFlip::Unknown;
    let mut sat = false;
    for lit in clause {
        if lit.eval(assignment[lit.var]) {
            sat = true;
            impact_of_flip = match impact_of_flip {
                ImpactOfFlip::Unknown => ImpactOfFlip::BreakableBy(lit.var),
                ImpactOfFlip::MakeableBy(_) => ImpactOfFlip::BreakableBy(lit.var),
                ImpactOfFlip::BreakableBy(_) => ImpactOfFlip::None,
                ImpactOfFlip::None => ImpactOfFlip::None
            }
        } else {
            impact_of_flip = match impact_of_flip {
                ImpactOfFlip::Unknown => {
                    ImpactOfFlip::MakeableBy(vec![lit.var])
                }
                ImpactOfFlip::MakeableBy(mut lits) => {
                    lits.push(lit.var);
                    ImpactOfFlip::MakeableBy(lits)
                }
                ImpactOfFlip::BreakableBy(lit) => ImpactOfFlip::BreakableBy(lit),
                ImpactOfFlip::None => ImpactOfFlip::None
            }
        }
    }

    ClauseState { impact_of_flip, sat }
}
