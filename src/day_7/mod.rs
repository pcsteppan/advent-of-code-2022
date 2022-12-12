use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

#[derive(Clone, Default, Debug)]
struct Directory {
    filesizes: i32,
    parent_directory: Option<Rc<RefCell<Directory>>>,
    sub_directories: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            filesizes: 0,
            parent_directory: None,
            sub_directories: HashMap::new(),
        }
    }

    fn set_filesize(&mut self, n: i32) {
        self.filesizes = n;
    }

    fn get_filesize(&self) -> i32 {
        self.filesizes
            + self
                .sub_directories
                .iter()
                .map(|(k, v)| v.borrow().get_filesize())
                .sum::<i32>()
    }

    fn get_aggregated_sizes_of_directories_below_threshold_size(
        &self,
        threshold_size: i32,
    ) -> (i32, i32) {
        let size = self
            .sub_directories
            .iter()
            .map(|(k, v)| {
                v.borrow()
                    .get_aggregated_sizes_of_directories_below_threshold_size(threshold_size)
            })
            .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
            .unwrap_or((0, 0));

        let total_directory_size = size.0 + self.filesizes;
        let mut running_total_under_threshold = size.1;

        if total_directory_size <= threshold_size {
            running_total_under_threshold += total_directory_size;
        }

        (total_directory_size, running_total_under_threshold)
    }

    fn get_smallest_directory_size_above_threshold(&self, threshold_size: i32) -> i32 {
        let sub_directory_sizes = self.sub_directories.iter().map(|(_, v)| {
            v.borrow()
                .get_smallest_directory_size_above_threshold(threshold_size)
        });

        let min_viable_subdirectory = sub_directory_sizes
            .clone()
            .filter(|size| size >= &threshold_size)
            .min()
            .unwrap_or(0);

        if min_viable_subdirectory > 0 {
            min_viable_subdirectory
        } else {
            sub_directory_sizes.sum::<i32>() + self.filesizes
        }
    }
}

pub fn solve_problem_part_one() {
    let file = fs::read_to_string("./src/day_7/input.txt").expect("Could not Parse File");

    let root = parse_input_to_directory(&file);
    let answer = root
        .borrow()
        .get_aggregated_sizes_of_directories_below_threshold_size(100_000);

    println!("{}", answer.1);
}

pub fn solve_problem_part_two() {
    let file = fs::read_to_string("./src/day_7/input.txt").expect("Could not Parse File");

    let root = parse_input_to_directory(&file);
    let total_space = 70000000;
    let space_required = 30000000;
    let minimum_space_to_free = space_required - (total_space - root.borrow().get_filesize());
    let answer = root
        .borrow()
        .get_smallest_directory_size_above_threshold(minimum_space_to_free);

    println!("{answer}");
}

fn parse_input_to_directory(file: &str) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new()));
    let mut current_directory = root.clone();

    file.split("\r\n").for_each(|line| {
        let command = line.split(' ').next().unwrap();

        if line.starts_with("$ ls") || line.eq("") {
            // do nothing
        } else if line.starts_with("dir") {
            let mut new_directory = Directory::new();
            let new_directory_name = line.split(" ").last().unwrap();

            new_directory.parent_directory = Some(Rc::clone(&current_directory));

            current_directory.borrow_mut().sub_directories.insert(
                new_directory_name.to_string(),
                Rc::new(RefCell::new(new_directory)),
            );
        } else if line.starts_with("$ cd") {
            let new_directory_name = line.split("cd ").last().unwrap();

            match new_directory_name {
                ".." => {
                    let current_parent =
                        current_directory.borrow().parent_directory.clone().unwrap();
                    current_directory = current_parent;
                }
                "/" => {}
                _ => {
                    let directory_to_move_to = current_directory
                        .borrow()
                        .sub_directories
                        .get(new_directory_name)
                        .unwrap()
                        .clone();

                    current_directory = directory_to_move_to;
                }
            }
        } else {
            let filesize = line.split(" ").next().unwrap().parse::<i32>().unwrap();

            current_directory.borrow_mut().filesizes += filesize;
        }
    });

    root
}
