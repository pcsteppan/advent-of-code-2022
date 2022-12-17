use std::fs;

enum Instruction {
    NOOP,
    ADDX(i32),
}

pub fn solve_problem_part_two() {
    let file = fs::read_to_string("./src/day_10/input.txt").expect("Could not parse file");
    let instructions: Vec<Instruction> = file
        .split("\r\n")
        .map(|line| {
            let mut tokens = line.split(" ");
            let first_token = tokens.next().unwrap();
            match first_token {
                "noop" => Instruction::NOOP,
                "addx" => Instruction::ADDX(tokens.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unexpected instruction"),
            }
        })
        .collect();
    let register = execute_instructions(instructions);

    let pixels: Vec<bool> = vec![1]
        .into_iter()
        .chain(register.into_iter())
        .enumerate()
        .map(|(i, register_value)| {
            (register_value - 1..register_value + 2).contains(&((i as i32) % 40))
        })
        .collect();

    for row in 0..6 {
        let start_index = row * 40;
        let end_index = start_index + 40;
        let pixel_row: String = pixels.as_slice()[start_index..end_index]
            .iter()
            .map(|sprite_present| if *sprite_present { '#' } else { '.' })
            .collect::<String>();

        println!("{pixel_row}");
    }
}

pub fn solve_problem_part_one() {
    let file = fs::read_to_string("./src/day_10/input.txt").expect("Could not parse file");
    let instructions: Vec<Instruction> = file
        .split("\r\n")
        .map(|line| {
            let mut tokens = line.split(" ");
            let first_token = tokens.next().unwrap();
            match first_token {
                "noop" => Instruction::NOOP,
                "addx" => Instruction::ADDX(tokens.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unexpected instruction"),
            }
        })
        .collect();

    let register = execute_instructions(instructions);

    let answer: i32 = (0..6)
        .map(|i| i * 40 + 20)
        .map(|i| register[i - 2] * (i as i32))
        .sum();

    println!("{answer}");
}

#[test]
fn test() {
    let instructions = vec![
        Instruction::NOOP,
        Instruction::ADDX(3),
        Instruction::ADDX(-5),
    ];

    let cycles = execute_instructions(instructions);

    assert_eq!(cycles[0], 1);
    assert_eq!(cycles[1], 1);
    assert_eq!(cycles[2], 4);
    assert_eq!(cycles[3], 4);
    assert_eq!(cycles[4], -1);
}

// Returns a register of the data
fn execute_instructions(instructions: Vec<Instruction>) -> Vec<i32> {
    let mut current_value = 1;
    instructions
        .into_iter()
        .fold(vec![], |mut acc, current_instruction| {
            acc.push(current_value);
            match current_instruction {
                Instruction::ADDX(x) => {
                    current_value += x;
                    acc.push(current_value);
                }
                _ => (),
            }
            acc
        })
}
