use std::fs;

pub fn solve_problem() {
    let file = fs::read_to_string("./src/day_one/input.txt").expect("Could not Parse File");
    let mut calorie_sums: Vec<i32> = file
        .split("\r\n\r\n")
        .map(|chunk| {
            chunk
                .split("\r\n")
                .map(|calorie_amt| calorie_amt.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();

    calorie_sums.sort_by(|a, b| b.cmp(a));

    // Answer 1
    println!("{}", calorie_sums[0]);
    // Answer 2
    println!("{}", calorie_sums.iter().take(3).sum::<i32>());
}
