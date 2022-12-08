use std::{collections::HashSet, fs};

pub fn solve_problem_part_one() {
    solve_problem(4);
}

pub fn solve_problem_part_two() {
    solve_problem(14);
}

pub fn solve_problem(token_length: usize) {
    let file = fs::read_to_string("./src/day_6/input.txt").expect("Could not Parse File");
    let stream: Vec<char> = file.chars().collect();

    for head_index in token_length..stream.len() {
        let sub_stream = stream.as_slice()[head_index - token_length..head_index].iter();
        let potential_token: HashSet<&char> = HashSet::from_iter(sub_stream);

        if potential_token.len() == token_length {
            println!("{head_index}");
            break;
        }
    }
}
