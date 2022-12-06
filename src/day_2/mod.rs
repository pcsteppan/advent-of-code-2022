use std::{fs, ops::Deref};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Shape {
    fn from_char(char: char) -> Shape {
        match char {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            c => panic!("Unexpected conversion from char {c} to Shape"),
        }
    }

    fn from_i32(num: i32) -> Shape {
        // rem_euclid is like '%' but it will also wrap negative numbers as expected
        let indexed_num = num.rem_euclid(3);
        match indexed_num {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            n => panic!("Unexpected conversion from number {n} to Shape"),
        }
    }
}

pub fn solve_problem_part_one() {
    let file = fs::read_to_string("./src/day_2/input.txt").expect("Could not Parse File");

    let total_score: i32 = file
        .split("\r\n")
        .map(|game| {
            let game_shapes: Vec<Shape> = game
                .split(" ")
                .map(|shape_input| Shape::from_char(shape_input.chars().next().unwrap()))
                .collect();

            let our_shape = game_shapes.last().unwrap();
            let opponents_shape = game_shapes.first().unwrap();

            let shape_as_score = our_shape.deref().clone() as i32 + 1;
            let versus_as_score = versus_to_score(our_shape, opponents_shape);

            shape_as_score + versus_as_score
        })
        .sum();

    println!("Total Score: {}", total_score);
}

pub fn solve_problem_part_two() {
    let file = fs::read_to_string("./src/day_2/input.txt").expect("Could not Parse File");

    let total_score: i32 = file
        .split("\r\n")
        .map(|game| {
            let (our_shape, opponents_shape) = shapes_from_input(game);

            let shape_as_score = our_shape as i32 + 1;
            let versus_as_score = versus_to_score(&our_shape, &opponents_shape);

            shape_as_score + versus_as_score
        })
        .sum();

    println!("Total Score: {}", total_score);
}

fn shapes_from_input(input: &str) -> (Shape, Shape) {
    let opponents_shape = Shape::from_char(input.chars().next().unwrap());
    let our_indicator: char = input.chars().nth(2).unwrap();

    let our_shape = match our_indicator {
        'Y' => opponents_shape,
        'X' => Shape::from_i32(opponents_shape as i32 - 1),
        'Z' => Shape::from_i32(opponents_shape as i32 + 1),
        i => panic!("Unexpected indicator '{i}', only expects X, Y, and Z"),
    };

    (our_shape, opponents_shape)
}

fn versus_to_score(our_shape: &Shape, opponents_shape: &Shape) -> i32 {
    return match (our_shape, opponents_shape) {
        (Shape::Scissors, Shape::Rock) => 0,
        (Shape::Rock, Shape::Scissors) => 6,
        (our_shape, opponents_shape) => {
            if our_shape > opponents_shape {
                6
            } else if our_shape == opponents_shape {
                3
            } else {
                0
            }
        }
    };
}
