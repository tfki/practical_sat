use std::fmt::{Debug, Display, Formatter};
use std::iter::{Enumerate, Skip};
use std::ops::{Index, IndexMut};
use bitvec::prelude::BitVec;
use bitvec::ptr::{BitRef, Mut};

use rand::random;
use solver::literal::Lit;

pub struct Assignment {
    inner: BitVec,
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
            match *val {
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
            inner: (0..num_vars).map(|_| random::<bool>()).collect(),
        }
    }

    pub fn randomize(&mut self) {
        self.inner.iter_mut().for_each(|mut val| *val = random());
    }
}

impl IntoIterator for Assignment {
    type Item = Lit;
    type IntoIter = AssignmentIterator;

    fn into_iter(self) -> Self::IntoIter {
        AssignmentIterator { assignment_iter: self.inner.into_iter().enumerate().skip(1) }
    }
}

impl Index<usize> for Assignment {
    type Output = BitRef;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Assignment {
    fn index_mut(&mut self, index: usize) -> BitRef<Mut> {
        self.inner.get_mut(index).unwrap()
    }
}

pub struct AssignmentIterator {
    assignment_iter: Skip<Enumerate<IntoIter<bool>>>,
}

impl Iterator for AssignmentIterator {
    type Item = Lit;

    fn next(&mut self) -> Option<Self::Item> {
        self.assignment_iter.next().map(|(id, val)| Lit { id: id as u32, negated: !val })
    }
}
