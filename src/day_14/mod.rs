use std::{fmt::Display, fs};

type Vec2 = (usize, usize);

struct State {
    map: Vec<Vec<bool>>,
    width_offset: usize,
    has_infinite_floor: bool,
}

type Line = (Vec2, Vec2);

impl State {
    fn from_lines(lines: &Vec<Line>, infinite_floor: bool) -> Self {
        let containing_box = lines.iter().fold((500, 500, 0), |acc, line| {
            (
                acc.0.min(line.0 .0).min(line.1 .0),
                acc.1.max(line.0 .0).max(line.1 .0),
                acc.2.max(line.0 .1).max(line.1 .1),
            )
        });

        let mut height = containing_box.2 + 1;
        let mut width = containing_box.1 - containing_box.0 + 1;
        let width_offset = containing_box.0;

        // hack to widen simulation space when infinite floor is enabled
        if infinite_floor {
            height += 2;
            width += 1000;
        }

        let mut state = vec![vec![false; width]; height];

        for line in lines {
            let line = if infinite_floor {
                ((line.0 .0 + 500, line.0 .1), (line.1 .0 + 500, line.1 .1))
            } else {
                *line
            };
            if line.0 .0 == line.1 .0 {
                let mut ys = [line.0 .1, line.1 .1];
                ys.sort();
                for y in ys[0]..=ys[1] {
                    state[y][line.0 .0 - width_offset] = true;
                }
            } else {
                let mut xs = [line.0 .0, line.1 .0];
                xs.sort();
                for x in xs[0]..=xs[1] {
                    state[line.0 .1][x - width_offset] = true;
                }
            }
        }

        return State {
            map: state,
            width_offset: width_offset,
            has_infinite_floor: infinite_floor,
        };
    }

    fn get_next_viable_position(&self, position: &Vec2) -> Option<Vec2> {
        if !self.is_position_in_bounds_of_map(&(position.0 as i32, position.1 as i32)) {
            return None;
        }

        let new_position_offsets: Vec<(i32, i32)> = vec![(0, 1), (-1, 1), (1, 1), (0, 0)];

        let new_position: Option<Vec2> = new_position_offsets
            .iter()
            .map(|offset| {
                (
                    (offset.0 + position.0 as i32),
                    (offset.1 + position.1 as i32),
                )
            })
            .filter(|potential_future_position| {
                let potential_future_position_usize = (
                    potential_future_position.0 as usize,
                    potential_future_position.1 as usize,
                );
                !self.is_position_in_bounds_of_map(potential_future_position)
                    || self.is_position_available(&potential_future_position_usize)
            })
            .map(|position_i32| (position_i32.0 as usize, position_i32.1 as usize))
            .next();

        match new_position {
            None => None,
            Some(new_position) => {
                if *position == new_position {
                    Some(*position)
                } else {
                    self.get_next_viable_position(&new_position)
                }
            }
        }
    }

    fn is_position_in_bounds_of_map(&self, position: &(i32, i32)) -> bool {
        if position.0 < 0 || position.0 >= (self.map.first().unwrap().len() as i32) {
            return false;
        }

        let position = (position.0 as usize, position.1 as usize);
        (0..self.map.first().unwrap_or(&Vec::new()).len()).contains(&position.0)
            && (0..self.map.len()).contains(&position.1)
    }

    fn is_position_occupied(&self, position: &Vec2) -> bool {
        self.map[position.1][position.0]
            || (self.has_infinite_floor && position.1 == self.map.len() - 1)
    }

    fn is_position_available(&self, position: &Vec2) -> bool {
        !self.is_position_occupied(position)
    }

    fn set_occupied(&mut self, position: Vec2) {
        self.map[position.1][position.0] = true;
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map_as_string: Vec<String> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect();
        write!(f, "{}", map_as_string.join("\r\n"))
    }
}

fn lines_from_string(string: String) -> Vec<Line> {
    string
        .split("\r\n")
        .map(|line| {
            let positions = line
                .split(" -> ")
                .map(|position| {
                    let mut parsed_position = position.split(",");
                    (
                        parsed_position.next().unwrap().parse::<usize>().unwrap(),
                        parsed_position.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<Vec2>>();

            positions
                .windows(2)
                .map(|window| (window[0], window[1]))
                .collect::<Vec<Line>>()
        })
        .flatten()
        .collect()
}

fn solve_problem(infinite_floor: bool) {
    let input_string = fs::read_to_string("./src/day_14/input.txt").expect("Failed to parse file");
    let lines = lines_from_string(input_string);
    let mut state = State::from_lines(&lines, infinite_floor);

    let offset = if infinite_floor { 1000 } else { 500 };
    let mut sand_count = 0;
    loop {
        let new_potential_sand_position =
            state.get_next_viable_position(&(offset - state.width_offset, 0));

        if let Some(new_sand_position) = new_potential_sand_position {
            state.set_occupied(new_sand_position);
            sand_count += 1;
        } else {
            break;
        }
    }

    println!("{}", sand_count);
}

pub fn solve_problem_part_one() {
    solve_problem(false);
}

pub fn solve_problem_part_two() {
    solve_problem(true);
}

#[test]
fn test_from_lines() {
    let lines = vec![
        ((498, 4), (498, 6)),
        ((498, 6), (496, 6)),
        ((503, 4), (502, 4)),
        ((502, 4), (502, 9)),
        ((502, 9), (494, 9)),
    ];
    let mut state = State::from_lines(&lines, false);
    println!("{}", state);

    let mut sand_count = 0;
    loop {
        let new_potential_sand_position =
            state.get_next_viable_position(&(500 - state.width_offset, 0));

        if let Some(new_sand_position) = new_potential_sand_position {
            state.set_occupied(new_sand_position);
            sand_count += 1;
        } else {
            break;
        }
    }

    assert_eq!(sand_count, 24);
}

#[test]
fn test_lines_from_string() {
    let line_string = String::from("531,95 -> 536,95\r\n527,102 -> 527,106 -> 523,106 -> 523,111 -> 540,111 -> 540,106 -> 533,106 -> 533,102");

    let lines: Vec<Line> = lines_from_string(line_string);

    let test_lines = vec![
        ((531, 95), (536, 95)),
        ((527, 102), (527, 106)),
        ((527, 106), (523, 106)),
        ((523, 106), (523, 111)),
        ((523, 111), (540, 111)),
        ((540, 111), (540, 106)),
        ((540, 106), (533, 106)),
        ((533, 106), (533, 102)),
    ];

    assert_eq!(lines, test_lines);
}
