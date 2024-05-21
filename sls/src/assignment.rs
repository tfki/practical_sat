use std::fmt::{Debug, Display, Formatter};
use std::iter::{Enumerate, Skip};
use std::ops::{Index, IndexMut};
use std::vec::IntoIter;

use rand::random;
use solver::variable::Var;

#[derive(Clone)]
pub struct Assignment {
    inner: Vec<bool>,
}

impl Debug for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, val) in self.inner.iter().enumerate() {
            writeln!(f, "{id} = {val}")?;
        }
        Ok(())
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v ")?;
        for (id, val) in self.inner.iter().enumerate() {
            match val {
                true => write!(f, "{id} ")?,
                false => write!(f, "-{id} ")?,
            }
        }
        Ok(())
    }
}

impl Assignment {
    pub fn new_random(num_vars: usize) -> Self {
        Self {
            inner: (0..=num_vars).map(|_| random::<bool>()).collect(),
        }
    }

    pub fn randomize(&mut self) {
        for i in 0..self.inner.len() {
            self.inner[i] = random();
        }
    }
}

impl IntoIterator for Assignment {
    type Item = (Var, bool);
    type IntoIter = AssignmentIterator;


    fn into_iter(self) -> Self::IntoIter {
        AssignmentIterator { assignment_iter: self.inner.into_iter().enumerate().skip(1) }
    }
}

impl Index<Var> for Assignment {
    type Output = bool;
    
    fn index(&self, index: Var) -> &Self::Output {
        &self.inner[index.id as usize]
    }
}

impl IndexMut<Var> for Assignment {
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        &mut self.inner[index.id as usize]
    }
}

pub struct AssignmentIterator {
    assignment_iter: Skip<Enumerate<IntoIter<bool>>>,
}

impl Iterator for AssignmentIterator {
    type Item = (Var, bool);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.assignment_iter.next().map(|(id, val)| (Var { id: id as u32 }, val))
    }
}
