use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Skip, StepBy};
use std::path::Path;
use std::slice::Iter;

pub mod naive_one_hot;
pub mod smart_one_hot;
#[cfg(test)]
mod test;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cell {
    Occupied(u32),
    Vacant,
}

impl From<u32> for Cell {
    fn from(value: u32) -> Self {
        match value {
            0 => Cell::Vacant,
            x => Cell::Occupied(x),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Sudoku {
    pub n: u32,
    cells: Vec<Cell>,
}

impl Sudoku {
    pub fn parse(path: impl AsRef<Path>) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let mut lines = reader.lines();

        let n = lines.next().unwrap().unwrap().parse::<u32>().unwrap();

        let cells = lines
            .flat_map(|line| line.unwrap()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u32>>())
            .map(|num| num.into())
            .collect::<Vec<Cell>>();

        Sudoku { cells, n }
    }

    pub fn cell_mut(&mut self, x: u32, y: u32) -> &mut Cell {
        &mut self.cells[(y * self.n.pow(2) + x) as usize]
    }

    pub fn cell(&self, x: u32, y: u32) -> &Cell {
        &self.cells[(y * self.n.pow(2) + x) as usize]
    }

    pub fn row(&self, row_no: u32) -> Iter<Cell> {
        let start = self.n.pow(2) * row_no;
        let end = start + self.n.pow(2);

        self.cells[start as usize..end as usize].iter()
    }

    pub fn col(&self, col_no: u32) -> StepBy<Skip<Iter<Cell>>> {
        self.cells.iter().skip(col_no as usize).step_by(self.n.pow(2) as usize)
    }

    pub fn finished_and_correct(&self) -> bool {
        if self.cells.iter().any(|cell| matches!(cell, Cell::Vacant)) {
            return false;
        }

        for row in 0..self.n.pow(2) {
            let row = self.row(row);

            for val in 1..self.n.pow(2) {
                if !row.clone().any(|cell| matches!(cell, Cell::Occupied(x) if *x == val)) {
                    return false;
                }
            }
        }

        for col in 0..self.n.pow(2) {
            let col = self.col(col);

            for val in 1..self.n.pow(2) {
                if !col.clone().any(|cell| matches!(cell, Cell::Occupied(x) if *x == val)) {
                    return false;
                }
            }
        }

        for block_x in 0..self.n {
            for block_y in 0..self.n {
                let x_offset = block_x * self.n;
                let y_offset = block_y * self.n;

                'val_loop: for val in 1..=self.n.pow(2) {
                    for x in x_offset..(x_offset + self.n) {
                        for y in y_offset..(y_offset + self.n) {
                            if matches!(self.cell(x, y), Cell::Occupied(x) if *x == val) {
                                continue 'val_loop;
                            }
                        }
                    }
                    return false;
                }
            }
        }

        true
    }

    pub fn is_permutation_of(&self, other: &Sudoku) -> bool {
        if self.n != other.n {
            return false;
        }

        for x in 0..self.n.pow(2) {
            for y in 0..self.n.pow(2) {
                if let Cell::Occupied(other_val) = other.cell(x, y) {
                    if !matches!(self.cell(x, y), Cell::Occupied(val) if *val == *other_val) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
