fn main() {
    let problem_path = "assets/hidoku_3x3_1";
    let problem_string = std::fs::read_to_string(problem_path).unwrap();

    let hidoku = Hidoku::from(problem_string.clone());
    println!("{problem_string}");
    println!("{hidoku:?}");
    println!("{:#?}", PossibleValueGrid::from(hidoku));
}

#[derive(Debug)]
pub struct PossibleValueGrid {
    rows: Vec<Vec<Vec<bool>>>,
}

impl PossibleValueGrid {
    pub fn new(dimens: usize) -> Self {
        PossibleValueGrid {
            rows: vec![vec![vec![true; dimens.pow(2)]; dimens]; dimens],
        }
    }
}

impl From<Hidoku> for PossibleValueGrid {
    fn from(hidoku: Hidoku) -> Self {
        let dimens = hidoku.dimens();
        let mut possible_values = PossibleValueGrid::new(dimens);

        // values that are already set in hidoku cannot be a possible value for any other cell
        let preset_values = hidoku.rows.iter().flat_map(|row| row.iter().flatten()).collect::<Vec<&u32>>();

        for x in 0..dimens {
            for y in 0..dimens {
                for value in &preset_values {
                    possible_values.rows[y][x][((**value) - 1) as usize] = false;
                }
            }
        }

        possible_values
    }
}

#[derive(Debug, Clone)]
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
