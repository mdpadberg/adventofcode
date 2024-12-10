use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use std::{collections::HashMap, path::PathBuf};

fn main() {
    println!("{}", solve(read_data_for_day("7").unwrap()));
}

#[derive(Debug)]
enum Content {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
struct File {
    size: u64,
    name: String,
}

#[derive(Debug)]
struct Directory {
    path: PathBuf,
}

fn solve(input: String) -> u64 {
    let mut root: HashMap<PathBuf, Vec<Content>> = HashMap::new();
    let mut current_path = PathBuf::from("");
    for line in input.split("\n") {
        let parsed = line.split(" ").collect::<Vec<&str>>();
        match &parsed[..] {
            &["$", "cd", "/"] => {
                current_path = PathBuf::from("/");
                root.insert(current_path.clone(), Vec::new());
            }
            &["$", "cd", ".."] => {
                current_path.pop();
            }
            &["$", "cd", char] => {
                current_path.push(char);
                root.insert(current_path.clone(), Vec::new());
            }
            &["$", "ls"] => {}
            &["dir", _] => root
                .get_mut(&current_path)
                .unwrap()
                .push(Content::Directory(Directory {
                    path: current_path
                        .join(PathBuf::from(line.chars().skip(4).collect::<String>())),
                })),
            &[size, filename] => root
                .get_mut(&current_path)
                .unwrap()
                .push(Content::File(File {
                    size: size.parse::<u64>().unwrap(),
                    name: filename.to_string(),
                })),
            _ => {}
        }
    }
    let mut dirs: Vec<(String, u64)> = Vec::new();
    for (path, content) in &root {
        let dir = (
            String::from(path.to_str().unwrap()),
            calculate_size_for_a_dir_recursively(&path, &root),
        );
        dirs.push(dir);
    }
    let missing_space: u64 = 30000000
        - (70000000
            - dirs
                .iter()
                .filter(|dir| dir.0 == "/")
                .map(|dir| dir.1)
                .sum::<u64>());
    dirs.iter()
        .filter(|dir| dir.1 >= missing_space)
        .map(|dir| dir.1)
        .min()
        .unwrap()
}

fn calculate_size_for_a_dir_recursively(
    current_dir: &PathBuf,
    all_dirs: &HashMap<PathBuf, Vec<Content>>,
) -> u64 {
    let content_of_current_dir = all_dirs.get(current_dir).unwrap();
    let mut size_of_current_dir = 0;
    for content in content_of_current_dir {
        match content {
            Content::Directory(Directory { path }) => {
                size_of_current_dir += calculate_size_for_a_dir_recursively(path, &all_dirs)
            }
            Content::File(File { size, name }) => size_of_current_dir += size,
        }
    }
    size_of_current_dir
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(24933642, solve(read_test_data_for_day("7-1").unwrap()));
    }
}
