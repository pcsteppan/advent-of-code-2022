use std::{fs, ops::RangeTo};

#[derive(Debug)]
pub struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(PartialEq)]
pub enum MachineType {
    Type9000,
    Type9001,
}

pub fn solve_problem_part_one() {
    solve_problem(MachineType::Type9000);
}

pub fn solve_problem_part_two() {
    solve_problem(MachineType::Type9001);
}

pub fn solve_problem(machine_type: MachineType) {
    let file = fs::read_to_string("./src/day_5/input.txt").expect("Could not Parse File");

    let instructions: Vec<Instruction> = file
        .split("\r\n")
        .skip(10)
        .map(|instruction_str| {
            let mut tokens = instruction_str.split(' ');
            Instruction {
                quantity: tokens.nth(1).unwrap().parse::<usize>().unwrap(),
                from: tokens.nth(1).unwrap().parse::<usize>().unwrap(),
                to: tokens.nth(1).unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect();

    let initial_stacks: [Vec<char>; 9] = [
        vec!['S', 'P', 'W', 'N', 'J', 'Z'],
        vec!['T', 'S', 'G'],
        vec!['H', 'L', 'R', 'Q', 'V'],
        vec!['D', 'T', 'S', 'V'],
        vec!['J', 'M', 'B', 'D', 'T', 'Z', 'Q'],
        vec!['L', 'Z', 'C', 'D', 'J', 'T', 'W', 'M'],
        vec!['J', 'T', 'G', 'W', 'M', 'P', 'L'],
        vec!['H', 'Q', 'F', 'B', 'T', 'M', 'G', 'N'],
        vec!['W', 'Q', 'B', 'P', 'C', 'G', 'D', 'R'],
    ]
    .map(|each_stack| each_stack.into_iter().rev().collect());

    let stacks = instructions
        .iter()
        .fold(initial_stacks, |stacks: [Vec<char>; 9], instruction| {
            execute_instruction_on_stacks(instruction, stacks.clone(), &machine_type)
        });

    let answer: String = stacks
        .clone()
        .map(|stack| stack.last().unwrap().clone())
        .iter()
        .collect();

    println!("{answer}");
}

pub fn execute_instruction_on_stacks(
    instruction: &Instruction,
    stacks: [Vec<char>; 9],
    machine_type: &MachineType,
) -> [Vec<char>; 9] {
    let mut new_stacks = stacks.clone();
    let mut quantity = instruction.quantity;

    if machine_type == &MachineType::Type9000 {
        while quantity > 0 {
            let item = new_stacks[instruction.from - 1].pop().unwrap_or('_');
            if item == '_' {
                break;
            }
            new_stacks[instruction.to - 1].push(item);
            quantity -= 1;
        }
    } else if machine_type == &MachineType::Type9001 {
        let stack_height = new_stacks[instruction.from - 1].len();
        let quantity = std::cmp::min(instruction.quantity, stack_height);

        let mut crates_to_move = new_stacks[instruction.from - 1]
            .splice(stack_height - quantity..stack_height, None)
            .collect::<Vec<char>>();

        new_stacks[instruction.to - 1].append(&mut crates_to_move);
    }

    return new_stacks;
}
