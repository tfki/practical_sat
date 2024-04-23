use std::ffi::{c_int, c_uint};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Variable {
    pub id: c_uint,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Literal {
    pub var: Variable,
    pub negated: bool,
}

impl From<c_int> for Literal {
    fn from(value: c_int) -> Self {
        let negated = value < 0;
        let var = Variable{ id: value.abs() as u32 };

        Literal{ var, negated }
    }
}

impl From<Literal> for c_int {
    fn from(value: Literal) -> Self {
        value.var.id as c_int
    }
}
