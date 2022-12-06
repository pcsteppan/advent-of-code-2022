use std::{collections::HashSet, fs};

use regex::Regex;

pub fn solve_problem_part_one() {
    let file = fs::read_to_string("./src/day_3/input.txt").expect("Could not Parse File");

    let sum_of_priorities: i32 = file
        .trim()
        .split("\r\n")
        .map(|inventory| {
            let compartment_size = inventory.chars().count() / 2;
            let (compartment1, compartment2) = inventory.split_at(compartment_size);
            let compartments = [compartment1, compartment2];
            let common_item = find_common_item(&mut compartments.into_iter());

            item_to_priority(common_item)
        })
        .sum();

    println!("{sum_of_priorities}");
}

pub fn solve_problem_part_two() {
    let file = fs::read_to_string("./src/day_3/input.txt").expect("Could not Parse File");

    let separator = Regex::new(r"((?m:^.+\s){3})").expect("Invalid regex");

    let answer: i32 = separator
        .captures_iter(&file)
        .map(|capture| {
            let grouped_inventories = &capture[0].trim();
            let mut inventories = grouped_inventories.split("\r\n").into_iter();

            find_common_item(&mut inventories)
        })
        .map(|item| item_to_priority(item))
        .sum();

    println!("{answer}");
}

fn find_common_item(iter: &mut dyn Iterator<Item = &str>) -> char {
    let iter_sets: Vec<HashSet<char>> = iter
        .map(|items| HashSet::<char>::from_iter(items.chars()))
        .collect();

    iter_sets.as_slice()[1..]
        .iter()
        .fold(iter_sets.first().unwrap().clone(), |intersection, set| {
            intersection.intersection(set).copied().collect()
        })
        .iter()
        .next()
        .unwrap()
        .clone()
}

fn item_to_priority(item: char) -> i32 {
    if item.is_lowercase() {
        item as i32 - 96
    } else {
        item as i32 - 38
    }
}
