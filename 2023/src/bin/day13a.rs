use std::collections::HashMap;

use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("13").unwrap()));
}

fn solve(input: String) -> usize {
    let mountains = input
        .split("\n\n")
        .map(|mountain| parse_moutain(mountain))
        .collect::<Vec<HashMap<Direction, Vec<Vec<char>>>>>();
    mountains
        .iter()
        .map(|mountain| {
            check_if_for_mirrors(
                mountain.get(&Direction::VERTICAL).unwrap(),
                &Direction::VERTICAL,
            ) + check_if_for_mirrors(
                mountain.get(&Direction::HORIZONTAL).unwrap(),
                &Direction::HORIZONTAL,
            )
        })
        .sum()
}

fn parse_moutain(input: &str) -> HashMap<Direction, Vec<Vec<char>>> {
    let horizontal = input
        .split("\n")
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut vertical = vec![vec![' '; horizontal.len()]; horizontal[0].len()];
    for y in 0..horizontal.len() {
        for x in 0..horizontal[y].len() {
            vertical[x][y] = horizontal[y][x]
        }
    }
    let mut map = HashMap::new();
    map.insert(Direction::HORIZONTAL, horizontal);
    map.insert(Direction::VERTICAL, vertical);
    map
}

fn check_if_for_mirrors(mountain: &Vec<Vec<char>>, direction: &Direction) -> usize {
    let mut result = 0;
    for (i, pair) in mountain.windows(2).enumerate() {
        // Start of possible mirror for example 5 and 6
        if pair[0] == pair[1] {
            // Check if its full mirror by looping from pair (5,6) to end of the mountain, for example
            // 123456789
            //  <<<<>>>>
            // #.##..##.
            // ..#.##.#.
            // ##......#
            // ##......#
            // ..#.##.#.
            // ..##..##.
            // #.#.##.#.
            let mut j: usize = 0;
            let is_full_mirror = loop {
                let is_negative_and_out_of_mountain_view = ((i as isize) - j as isize) < 0;
                let first = if is_negative_and_out_of_mountain_view {
                    None
                } else {
                    mountain.get(i - j)
                };
                // None is always a full mirror match, because that means that its the end of the mountain view on one of the sides
                match (first, mountain.get(i + j + 1)) {
                    (None, None) => break true,
                    (None, Some(_)) => break true,
                    (Some(_), None) => break true,
                    (Some(a), Some(b)) => {
                        if a != b {
                            break false;
                        } else {
                            j += 1;
                        }
                    }
                }
            };
            if is_full_mirror {
                match direction {
                    Direction::HORIZONTAL => result += (i + 1) * 100,
                    Direction::VERTICAL => result += i + 1,
                }
            }
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    HORIZONTAL,
    VERTICAL,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn parse_moutain_test() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
        let mut map = HashMap::new();
        map.insert(
            Direction::HORIZONTAL,
            vec![
                vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'],
            ],
        );
        map.insert(
            Direction::VERTICAL,
            vec![
                vec!['#', '.', '#', '#', '.', '.', '#'],
                vec!['.', '.', '#', '#', '.', '.', '.'],
                vec!['#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.'],
                vec!['.', '#', '.', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '#', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '#', '.', '.', '#', '#', '#'],
                vec!['.', '.', '#', '#', '.', '.', '.'],
            ],
        );
        assert_eq!(map, parse_moutain(input));
    }

    #[test]
    fn solvetest() {
        assert_eq!(405, solve(read_test_data_for_day("13-0").unwrap()));
    }
}
