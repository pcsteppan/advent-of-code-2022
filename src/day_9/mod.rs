use std::{collections::HashSet, fs, ops::Add};

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::N => Vec2(0, 1),
            Direction::E => Vec2(1, 0),
            Direction::S => Vec2(0, -1),
            Direction::W => Vec2(-1, 0),
        }
    }
}

type Command = (Direction, i16);

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec2(i16, i16);
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Default, Debug)]
struct StringState {
    head: Vec2,
    tail: Vec<Vec2>,
    tail_length: i8,
}

// future improvement: implement IntoIter which returns all knots from StringState
impl StringState {
    fn new(tail_length: i8) -> StringState {
        StringState {
            tail: (0..tail_length).map(|_| Vec2(0, 0)).collect(),
            ..StringState::default()
        }
    }

    fn process_command(&mut self, command: &Command) -> Vec<Vec2> {
        let mut distance = command.1;
        let mut positions: Vec<Vec2> = vec![];
        while distance > 0 {
            self.update_head_position(&command.0);
            for i in 0..self.tail.len() {
                match i {
                    0 => {
                        Self::update_tail_position(&mut self.head, &mut self.tail[i]);
                    }
                    _ => {
                        let mut pseudo_head = self.tail[i - 1];
                        Self::update_tail_position(&mut pseudo_head, &mut self.tail[i]);
                    }
                }
            }

            positions.push(self.tail.last().unwrap().clone());
            distance -= 1;
        }
        positions
    }

    fn update_head_position(&mut self, direction: &Direction) {
        self.head = self.head + direction.to_vec2();
    }

    fn update_tail_position(head: &mut Vec2, tail: &mut Vec2) {
        let x_delta = head.0 - tail.0;
        let y_delta = head.1 - tail.1;

        if x_delta.abs() >= 2 || y_delta.abs() >= 2 {
            tail.0 = tail.0 + x_delta.signum();
            tail.1 = tail.1 + y_delta.signum();
        }
    }
}

pub fn solve_problem_part_one() {
    solve_problem(1);
}

pub fn solve_problem_part_two() {
    solve_problem(9);
}

fn solve_problem(tail_length: i8) {
    let file = fs::read_to_string("./src/day_9/input.txt").expect("Could not Parse File");

    let commands: Vec<Command> = file
        .split("\r\n")
        .into_iter()
        .map(|line| {
            let mut input = line.split(" ");
            let (input_direction, input_distance) = (
                input.next().unwrap(),
                input.next().unwrap().parse::<i16>().unwrap(),
            );

            let direction = match input_direction {
                "U" => Direction::N,
                "R" => Direction::E,
                "D" => Direction::S,
                "L" => Direction::W,
                _ => panic!("Unexpected direction"),
            };

            (direction, input_distance)
        })
        .collect();

    let string_state = StringState::new(tail_length);

    let unique_positions: HashSet<Vec2> = HashSet::from_iter(
        commands
            .iter()
            .fold((string_state, vec![]), |mut acc, curr| {
                let mut new_results = acc.0.process_command(curr);
                acc.1.append(&mut new_results);
                acc
            })
            .1
            .into_iter(),
    );

    println!("{}", unique_positions.len());
}

#[test]
fn test_part_one() {
    let test_commands = vec![
        (Direction::E, 4),
        (Direction::N, 4),
        (Direction::W, 3),
        (Direction::S, 1),
        (Direction::E, 4),
        (Direction::S, 1),
        (Direction::W, 5),
        (Direction::E, 2),
    ];

    let string_state = StringState::new(1);

    let test: HashSet<Vec2> = HashSet::from_iter(
        test_commands
            .iter()
            .fold((string_state, vec![]), |mut acc, curr| {
                let mut new_results = acc.0.process_command(curr);
                acc.1.append(&mut new_results);
                acc
            })
            .1
            .into_iter(),
    );

    assert_eq!(test.len(), 13);
}

#[test]
fn test_part_two() {
    let test_commands = vec![
        (Direction::E, 4),
        (Direction::N, 4),
        (Direction::W, 3),
        (Direction::S, 1),
        (Direction::E, 4),
        (Direction::S, 1),
        (Direction::W, 5),
        (Direction::E, 2),
    ];

    let string_state = StringState::new(10);

    let test: HashSet<Vec2> = HashSet::from_iter(
        test_commands
            .iter()
            .fold((string_state, vec![]), |mut acc, curr| {
                acc.1.append(&mut acc.0.process_command(curr));
                acc
            })
            .1
            .into_iter(),
    );

    assert_eq!(test.len(), 1);
}
