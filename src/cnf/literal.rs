use std::ffi::{c_int, c_uint};
use std::ops::Neg;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Variable {
    pub id: c_uint,
}

impl Variable {
    pub fn new(id: c_uint) -> Self {
        Self { id }
    }
}

impl Neg for Variable {
    type Output = Literal;

    fn neg(self) -> Self::Output {
        Literal::new(self, true)
    }
}

impl From<Variable> for Literal {
    fn from(value: Variable) -> Self {
        Literal::new(value, false)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Literal {
    pub var: Variable,
    pub negated: bool,
}

impl Literal {
    pub fn new(var: Variable, negated: bool) -> Self {
        Self { var, negated }
    }

    pub fn clause_end() -> Self {
        Self {
            var: Variable::new(0),
            negated: false
        }
    }
}

impl Neg for Literal {
    type Output = Literal;

    fn neg(mut self) -> Self::Output {
        self.negated = !self.negated;

        self
    }
}


impl From<c_int> for Literal {
    fn from(value: c_int) -> Self {
        let negated = value < 0;
        let var = Variable{ id: value.unsigned_abs() };

        Literal{ var, negated }
    }
}

impl From<Literal> for c_int {
    fn from(value: Literal) -> Self {
        match value.negated {
            true => -(value.var.id as i32), 
            false => value.var.id as i32, 
        }
    }
}
