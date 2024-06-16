use crate::Hidoku;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn all_coords(dimens: usize, hidoku: &Hidoku) -> Vec<Coord> {
    (0..dimens)
        .flat_map(|x|
            (0..dimens)
                .map(move |y| Coord { x, y }))
        .filter(|coord| hidoku.get(coord.x, coord.y).is_none())
        .collect()
}

pub fn get_possible_coords(hidoku: &Hidoku) -> Option<Vec<(u32, Vec<Coord>)>> {
    let dimens = hidoku.dimens();
    let dimens_sq = hidoku.dimens() * hidoku.dimens();

    let mut value_coords = vec![all_coords(dimens, hidoku); dimens_sq];
    let mut value_coord_set = vec![false; dimens_sq];

    for x in 0..dimens {
        for y in 0..dimens {
            if let Some(value) = hidoku.get(x, y) {
                value_coords[value as usize - 1] = vec![Coord { x, y }];
                value_coord_set[value as usize - 1] = true;
            }
        }
    }

    // left to right
    let mut iter_mut = value_coords.iter_mut().peekable();
    let mut value = 1;
    loop {
        if let Some(current) = iter_mut.next() {
            if let Some(next) = iter_mut.peek_mut() {
                // !value_coord_set[next] !!!
                if !value_coord_set[value] {
                    let new_next = intersection(next,
                                                &current.iter()
                                                    .flat_map(|coord| coord.neighbors_clipped(dimens))
                                                    .filter(|coord| hidoku.get(coord.x, coord.y).is_none())
                                                    .collect::<Vec<Coord>>());

                    **next = new_next;
                }
            } else {
                break;
            }
        }
        value += 1;
    }

    // right to left
    value_coord_set.reverse();

    let mut iter_mut = value_coords.iter_mut().rev().peekable();
    let mut value = 1;
    loop {
        if let Some(current) = iter_mut.next() {
            if let Some(next) = iter_mut.peek_mut() {
                // !value_coord_set[next] !!!
                if !value_coord_set[value] {
                    let new_next = intersection(next,
                                                &current.iter()
                                                    .flat_map(|coord| coord.neighbors_clipped(dimens))
                                                    .filter(|coord| hidoku.get(coord.x, coord.y).is_none())
                                                    .collect::<Vec<Coord>>());

                    **next = new_next;
                }
            } else {
                break;
            }
        }
        value += 1;
    }

    if value_coords.iter().any(|coords| coords.is_empty()) {
        return None;
    }

    Some(value_coords.into_iter().enumerate().map(|(i, coords)| (i as u32 + 1, coords)).collect())
}

impl Coord {
    pub fn is_neighbor_of(&self, other: &Coord) -> bool {
        (self.x as i32 - other.x as i32).abs() <= 1
            && (self.y as i32 - other.y as i32).abs() <= 1
    }

    pub fn neighbors_clipped(&self, dimens: usize) -> Vec<Coord> {
        let mut neighbors = vec![];

        let x_minus_one = self.x.checked_sub(1);
        let x_plus_one = if self.x + 1 < dimens { Some(self.x + 1) } else { None };
        let y_minus_one = self.y.checked_sub(1);
        let y_plus_one = if self.y + 1 < dimens { Some(self.y + 1) } else { None };

        if let Some(x) = x_minus_one {
            neighbors.push(Coord { x, y: self.y });
        }
        if let Some(x) = x_plus_one {
            neighbors.push(Coord { x, y: self.y });
        }
        if let Some(y) = y_minus_one {
            neighbors.push(Coord { x: self.x, y });
        }
        if let Some(y) = y_plus_one {
            neighbors.push(Coord { x: self.x, y });
        }
        if let (Some(x), Some(y)) = (x_plus_one, y_plus_one) {
            neighbors.push(Coord { x, y });
        }
        if let (Some(x), Some(y)) = (x_plus_one, y_minus_one) {
            neighbors.push(Coord { x, y });
        }
        if let (Some(x), Some(y)) = (x_minus_one, y_plus_one) {
            neighbors.push(Coord { x, y });
        }
        if let (Some(x), Some(y)) = (x_minus_one, y_minus_one) {
            neighbors.push(Coord { x, y });
        }

        neighbors
    }
}

fn bfs_one_step(coords: Vec<Coord>, hidoku: &Hidoku) -> Vec<Coord> {
    let mut all_neighbors =
        coords.iter().flat_map(|coord| coord.neighbors_clipped(hidoku.dimens())).collect::<Vec<Coord>>();
    all_neighbors.sort();
    all_neighbors.dedup();
    all_neighbors.retain(|neighbor| hidoku.get(neighbor.x, neighbor.y).is_none());

    all_neighbors
}

fn intersection(a: &Vec<Coord>, b: &[Coord]) -> Vec<Coord> {
    let mut result = vec![];

    for x in a {
        if b.iter().any(|y| *y == *x) {
            result.push(*x);
        }
    }

    result
}
