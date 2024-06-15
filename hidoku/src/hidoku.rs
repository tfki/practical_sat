use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Hidoku {
    rows: Vec<Vec<Option<u32>>>,
}

impl Hidoku {
    pub fn dimens(&self) -> usize {
        self.rows.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u32> {
        self.rows[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: Option<u32>) {
        self.rows[y][x] = value;
    }

    pub fn is_permutation_of(&self, other: &Hidoku) -> bool {
        if self.dimens() != other.dimens() {
            return false;
        }

        if self.rows.iter().any(|row| row.iter().any(|cell| cell.is_none())) {
            return false;
        }

        let dimens = self.dimens();

        for x in 0..dimens {
            for y in 0..dimens {
                if let Some(other_value) = other.get(x, y) {
                    if other_value != self.get(x, y).unwrap() {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Debug for Hidoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for item in row {
                match item {
                    None => write!(f, "?, ")?,
                    Some(num) => write!(f, "{num}, ")?,
                }
            }
            if Some(row) != self.rows.last() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl From<String> for Hidoku {
    fn from(value: String) -> Self {
        let mut iter = value.split(':');

        let wh = iter.next().unwrap();
        let cells = iter.next().unwrap();

        let (w, h) = wh.split_once(',').map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())).unwrap();
        assert_eq!(w, h);

        let rows = cells.split(';');
        let result = Hidoku {
            rows:
            rows.filter_map(|row| {
                if row.is_empty() {
                    None
                } else {
                    Some(row.split(',')
                        .map(|entry| entry.parse::<u32>().unwrap())
                        .map(|number| if number == 0 { None } else { Some(number) })
                        .collect())
                }
            }).collect(),
        };

        assert_eq!(result.rows.len(), w as usize);
        for row in &result.rows {
            assert_eq!(row.len(), w as usize);
        }

        result
    }
}
