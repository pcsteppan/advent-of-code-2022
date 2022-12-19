use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::once,
};

type Vec2 = (usize, usize);
type Path = Vec<Vec2>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    index: Vec2,
    previous: Path,
}

pub fn solve_problem_part_one() {
    solve_problem(Mode::FromStart)
}

pub fn solve_problem_part_two() {
    solve_problem(Mode::FromLowestElevation);
}

#[derive(PartialEq)]
enum Mode {
    FromStart,
    FromLowestElevation,
}

fn solve_problem(mode: Mode) {
    let test_input = fs::read_to_string("./src/day_12/input.txt").expect("Could not parse file");

    let map: Vec<Vec<usize>> = test_input
        .replace('S', "a")
        .replace('E', "z")
        .split("\r\n")
        .map(|line| line.chars().map(|char| char as usize).collect())
        .collect();

    let linear_index_of_start = test_input.replace("\r\n", "").find('S').unwrap();
    let start_position =
        linear_index_to_grid_index(linear_index_of_start, map.first().unwrap().len());

    let linear_index_of_end = test_input.replace("\r\n", "").find('E').unwrap();
    let end_position = linear_index_to_grid_index(linear_index_of_end, map.first().unwrap().len());

    let mut visited = HashSet::<Vec2>::new();
    let mut frontier: VecDeque<Node> = VecDeque::new();
    frontier.push_back(Node {
        index: end_position,
        previous: vec![],
    });

    while !frontier.is_empty() {
        let curr_node: Node = frontier.pop_front().unwrap();
        if visited.contains(&curr_node.index) {
            continue;
        }

        if mode == Mode::FromStart && curr_node.index == start_position {
            println!("{}", curr_node.previous.len());
            break;
        } else if mode == Mode::FromLowestElevation
            && map[curr_node.index.1][curr_node.index.0] == 'a' as usize
        {
            println!("{}", curr_node.previous.len());
            break;
        }

        visited.insert(curr_node.index);
        let viable_neighbors = get_viable_neighbors(&map, curr_node.clone());
        if viable_neighbors.is_empty() {
            println!("no viable neighbors at {:?}", &curr_node.index);
        }
        frontier.append(&mut VecDeque::from(viable_neighbors));
    }
}

fn linear_index_to_grid_index(linear_index: usize, w: usize) -> Vec2 {
    (linear_index % w, linear_index / w)
}

fn get_viable_neighbors(map: &Vec<Vec<usize>>, node: Node) -> Vec<Node> {
    let curr_node_value = map[node.index.1][node.index.0];
    let index = node.clone().index;

    vec![(0, 1), (1, 0), (-1, 0), (0, -1)]
        .iter()
        .map(|offset| (index.0 as i32 + offset.0, index.1 as i32 + offset.1))
        .filter(|neighbor_position| {
            neighbor_position.0 >= 0
                && neighbor_position.0 < map.first().unwrap().len() as i32
                && neighbor_position.1 >= 0
                && neighbor_position.1 < map.len() as i32
                && ((map[neighbor_position.1 as usize][neighbor_position.0 as usize] as isize)
                    - curr_node_value as isize)
                    >= -1
        })
        .map(|valid_neighbor_position| Node {
            index: (
                valid_neighbor_position.0 as usize,
                valid_neighbor_position.1 as usize,
            ),
            previous: node
                .clone()
                .previous
                .into_iter()
                .chain(once(node.index))
                .collect(),
        })
        .collect()
}
