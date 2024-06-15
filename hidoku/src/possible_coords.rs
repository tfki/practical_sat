use crate::Hidoku;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn get_possible_coords(hidoku: &Hidoku) -> Option<Vec<(u32, Vec<Coord>)>> {
    let dimens = hidoku.dimens();
    let dimens_sq = hidoku.dimens() * hidoku.dimens();

    let mut values_with_known_coords = Vec::<(u32, Vec<Coord>)>::new();

    for x in 0..dimens {
        for y in 0..dimens {
            if let Some(value) = hidoku.get(x, y) {
                values_with_known_coords.push((value, vec![Coord { x, y }]));
            }
        }
    }

    values_with_known_coords.sort_by_key(|(number, _)| *number);

    // trivial rejects
    for i in 0..(values_with_known_coords.len() - 1) {
        let j = i + 1;

        let (a_num, a_coords) = &values_with_known_coords[i];
        let (b_num, b_coords) = &values_with_known_coords[j];

        // if a_num and b_num are neighbors, their fields must be neighbors too
        if (*a_num as i32 - *b_num as i32).abs() == 1 &&
            !a_coords.first().unwrap().is_neighbor_of(b_coords.first().unwrap()) {
            return None;
        }
    }

    let mut i = 0;
    loop {
        let j = i + 1;
        if j == values_with_known_coords.len() {
            break;
        }

        let (a_num, a_coords) = &values_with_known_coords[i];
        let (b_num, b_coords) = &values_with_known_coords[j];

        if (*a_num as i32 - *b_num as i32).abs() > 1 {
            let mut current_a = *a_num;
            let mut current_b = *b_num;

            let mut current_a_coords = a_coords.clone();
            let mut current_b_coords = b_coords.clone();

            if current_b < current_a {
                std::mem::swap(&mut current_a, &mut current_b);
                std::mem::swap(&mut current_a_coords, &mut current_b_coords);
            }

            let mut turn = true;
            while current_a != current_b {
                if turn {
                    current_a += 1;
                    current_a_coords = bfs_one_step(current_a_coords, hidoku);
                } else {
                    current_b -= 1;
                    current_b_coords = bfs_one_step(current_b_coords, hidoku);
                }
                turn = !turn;
            }

            let intersection = intersection(&current_a_coords, &current_b_coords);
            if intersection.is_empty() {
                return None;
            }

            values_with_known_coords.insert(j, (current_a, intersection));
        } else {
            i += 1;
        }
    }

    while let Some((number, possible_coords)) = values_with_known_coords.first() {
        if *number == 1 {
            break;
        } else {
            values_with_known_coords.insert(0, (number - 1, bfs_one_step(possible_coords.clone(), hidoku)));
        }
    }
    while let Some((number, possible_coords)) = values_with_known_coords.last() {
        if *number == dimens_sq as u32 {
            break;
        } else {
            values_with_known_coords.push((number + 1, bfs_one_step(possible_coords.clone(), hidoku)));
        }
    }

    Some(values_with_known_coords)
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
