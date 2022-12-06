use std::fs;

type Range = (i32, i32);

pub fn solve_problem_part_one() {
    let file = fs::read_to_string("./src/day_4/input.txt").expect("Could not Parse File");

    let answer = file
        .trim()
        .split("\r\n")
        .filter(|pair_input| {
            let (range1, range2) = ranges_from_input(pair_input);

            range_contains_range(range1, range2) || range_contains_range(range2, range1)
        })
        .count();

    println!("{answer}");
}

pub fn solve_problem_part_two() {
    let file = fs::read_to_string("./src/day_4/input.txt").expect("Could not Parse File");

    let answer = file
        .trim()
        .split("\r\n")
        .filter(|pair_input| {
            let (range1, range2) = ranges_from_input(pair_input);

            range_overlaps_range(range1, range2) || range_overlaps_range(range2, range1)
        })
        .count();

    println!("{answer}");
}

pub fn ranges_from_input(input: &str) -> (Range, Range) {
    let mut ranges = input.split(',');
    let mut range1 = ranges.next().unwrap().split("-");
    let range1 = (
        range1.next().unwrap().parse::<i32>().unwrap(),
        range1.next().unwrap().parse::<i32>().unwrap(),
    );
    let mut range2 = ranges.next().unwrap().split("-");
    let range2 = (
        range2.next().unwrap().parse::<i32>().unwrap(),
        range2.next().unwrap().parse::<i32>().unwrap(),
    );

    (range1, range2)
}

pub fn range_contains_range(range1: Range, range2: Range) -> bool {
    return range1.0 <= range2.0 && range1.1 >= range2.1;
}

pub fn range_overlaps_range(range1: Range, range2: Range) -> bool {
    return !(range1.0 > range2.1 || range1.1 < range2.0);
}
