use std::{collections::HashSet, fs, iter::Enumerate, str::Split};

type Hits = Vec<Vec<(usize, usize)>>;

pub fn solve_problem_part_one() {
    let file = fs::read_to_string("./src/day_8/input.txt").expect("Could not Parse File");
    let lines = file.split("\r\n");

    let forest_width = lines.clone().next().unwrap().chars().count() - 1;
    let forest_height = lines.clone().count() - 1;

    let forest: Vec<Vec<_>> = lines
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let raycast_forest: Hits = get_transformed_forest(&forest, false, false)
        .map(|(i, tree_row)| {
            find_visible_trees_in_row(&tree_row)
                .into_iter()
                .map(|hit| (i, hit as usize))
                .collect::<Vec<_>>()
        })
        .collect();

    let raycast_reverse_forest: Hits = get_transformed_forest(&forest, false, true)
        .map(|(i, tree_row)| {
            find_visible_trees_in_row(&tree_row)
                .into_iter()
                .map(|hit| (i, forest_width - hit as usize))
                .collect::<Vec<_>>()
        })
        .collect();

    let raycast_transposed_forest: Hits = get_transformed_forest(&forest, true, false)
        .map(|(i, tree_row)| {
            find_visible_trees_in_row(&tree_row)
                .into_iter()
                .map(|hit| (hit as usize, i))
                .collect::<Vec<_>>()
        })
        .collect();

    let raycast_reverse_transposed_forest: Hits = get_transformed_forest(&forest, true, true)
        .map(|(i, tree_row)| {
            find_visible_trees_in_row(&tree_row)
                .into_iter()
                .map(|hit| (forest_height - hit as usize, i))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let all_unique_hits: HashSet<(usize, usize)> = HashSet::from_iter(
        raycast_forest.into_iter().flatten().chain(
            raycast_reverse_forest
                .into_iter()
                .flatten()
                .chain(
                    raycast_transposed_forest
                        .into_iter()
                        .flatten()
                        .chain(
                            raycast_reverse_transposed_forest
                                .into_iter()
                                .flatten()
                                .collect::<Vec<_>>(),
                        )
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>(),
        ),
    );

    println!("{}", all_unique_hits.len());
}

// casts rays from left to right
fn find_visible_trees_in_row(tree_row: &Vec<i8>) -> Vec<u32> {
    tree_row
        .iter()
        .enumerate()
        .fold(vec![], |mut res: Vec<(u32, i8)>, (i, tree)| {
            if tree > &res.last().unwrap_or(&(0, -1)).1 {
                res.push((i as u32, *tree));
            }
            res
        })
        .iter()
        .map(|hit| hit.0)
        .collect()
}

fn get_transformed_forest(
    forest: &Vec<Vec<i8>>,
    should_transpose: bool,
    should_reverse: bool,
) -> Enumerate<std::vec::IntoIter<Vec<i8>>> {
    let forest = if should_transpose {
        transpose(forest.clone())
    } else {
        forest.clone()
    };

    forest
        .iter()
        .map(|tree_row| {
            if should_reverse {
                reverse(tree_row)
            } else {
                tree_row.clone()
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
}

fn reverse<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    vec.clone().into_iter().rev().collect::<Vec<T>>()
}

// adapted from https://users.rust-lang.org/t/rayon-transpose-of-vec-vec-t/62864/2
pub fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .into_iter()
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}
