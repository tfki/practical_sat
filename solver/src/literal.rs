use std::ops::Neg;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Lit {
    pub id: u32,
    pub negated: bool,
}

impl Lit {
    pub(crate) fn new(id: u32) -> Self {
        assert_ne!(id, 0);
        Lit { id, negated: false }
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
        Lit { id: value.unsigned_abs(), negated: value < 0 }
    }
}

impl From<Lit> for i32 {
    fn from(value: Lit) -> Self {
        match value.negated {
            true => -(value.id as i32),
            false => value.id as i32,
        }
    }
}
