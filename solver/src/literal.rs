use std::ops::Neg;

use crate::variable::Var;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Lit {
    pub var: Var,
    pub negated: bool,
}

impl Lit {
    pub(crate) fn new(id: u32) -> Self {
        assert_ne!(id, 0);
        Lit { var: Var { id }, negated: false }
    }

    pub fn eval(self, var_value: bool) -> bool {
        self.negated ^ var_value
    }
}

impl Neg for Lit {
    type Output = Lit;

    fn neg(mut self) -> Self::Output {
        self.negated = !self.negated;

        self
    }
}


impl From<i32> for Lit {
    fn from(value: i32) -> Self {
        Lit { var: Var { id: value.unsigned_abs() }, negated: value < 0 }
    }
}

impl From<Lit> for i32 {
    fn from(value: Lit) -> Self {
        match value.negated {
            true => -(value.var.id as i32),
            false => value.var.id as i32,
        }
    }
}
