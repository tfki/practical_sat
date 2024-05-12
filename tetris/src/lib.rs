use std::fmt::{Debug, Formatter};

pub mod naive;
#[cfg(test)]
mod test;

pub struct Input {
    pub height: u32,
    pub width: u32,
    pub num_i: u32,
    pub num_t: u32,
    pub num_l: u32,
    pub num_s: u32,
    pub num_o: u32,
}

pub struct Solution {
    pub grid: Vec<Vec<Option<CellContent>>>,
}

impl Solution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { grid: vec![vec![None; height as usize]; width as usize] }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.grid.is_empty() {
            return Ok(());
        }

        writeln!(f)?;
        for y in 0..self.grid[0].len() {
            for x in 0..self.grid.len() {
                match self.grid[x][y] {
                    None => write!(f, "  ")?,
                    Some(content) => write!(f, "{content:?} ")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CellContent {
    I,
    O,
    S,
    T,
    L,
}

impl CellContent {
    pub fn all_values() -> [CellContent; 5] {
        [CellContent::I, CellContent::O, CellContent::S, CellContent::T, CellContent::L]
    }

    pub fn offsets(self) -> &'static [(u32, u32)] {
        match self {
            CellContent::I => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            CellContent::O => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            CellContent::S => &[(1, 0), (2, 0), (0, 1), (1, 1)],
            CellContent::T => &[(0, 0), (1, 0), (2, 0), (1, 1)],
            CellContent::L => &[(0, 0), (0, 1), (1, 1), (2, 1)],
        }
    }

    pub fn excluded_other_anchor_offsets(self) -> &'static [(i32, i32)] {
        match self {
            CellContent::I => &[(0, 1), (0, 2), (0, 3)],
            CellContent::O => &[(-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1)],
            CellContent::S => &[(-1, 0), (-1, 1), (-2, 1), (1, 0), (0, 1)],
            CellContent::T => &[(-1, 0), (-2, 0), (-1, 1), (1, 0), (2, 0), (0, 1), (1, 1)],
            CellContent::L => &[(-1, 0), (-2, 0), (0, 1), (1, 1), (2, 1)],
        }
    }

    pub fn dimensions(self) -> (u32, u32) {
        match self {
            CellContent::I => (1, 4),
            CellContent::O => (2, 2),
            CellContent::S => (3, 2),
            CellContent::T => (3, 2),
            CellContent::L => (3, 2),
        }
    }
}
