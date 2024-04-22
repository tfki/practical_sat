use std::ffi::c_int;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Literal {
    pub id: c_int,
    pub negated: bool,
}

impl From<c_int> for Literal {
    fn from(value: c_int) -> Self {
        let negated = value < 0;
        let var = value.abs();

        Literal{ id: var, negated }
    }
}
