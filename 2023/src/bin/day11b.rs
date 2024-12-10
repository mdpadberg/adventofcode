use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("11").unwrap()));
}

fn solve(input: String) -> usize {
    let space = parse_space(input);
    let mut galaxies = get_galaxies_from_space(&space);
    expand_space(&space, &mut galaxies);
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(galaxy1, galaxy2)| galaxy1.x.abs_diff(galaxy2.x) + galaxy1.y.abs_diff(galaxy2.y))
        .sum()
}

fn parse_space(input: String) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_galaxies_from_space(space: &Vec<Vec<char>>) -> Vec<Galaxy> {
    space
        .iter()
        .enumerate()
        .flat_map(|(y, chars)| {
            chars
                .iter()
                .enumerate()
                .filter_map(|(x, value)| {
                    if value == &'#' {
                        Some(Galaxy { x, y })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect::<Vec<Galaxy>>()
}

fn expand_space(space: &Vec<Vec<char>>, galaxies: &mut Vec<Galaxy>) {
    let (max_y, max_x) = (space.len(), space[0].len());
    let (empty_y, empty_x) = (
        (0..max_y)
            .filter(|&row_number| space[row_number].iter().all(|&char| char == '.'))
            .collect::<Vec<usize>>(),
        (0..max_x)
            .filter(|&col_number| (0..max_y).all(|row_number| space[row_number][col_number] == '.'))
            .collect::<Vec<usize>>(),
    );
    empty_y.iter().rev().for_each(|y| {
        galaxies.iter_mut().for_each(|galaxy| {
            if &galaxy.y > y {
                galaxy.y += 999999
            }
        })
    });
    empty_x.iter().rev().for_each(|x| {
        galaxies.iter_mut().for_each(|galaxy| {
            if &galaxy.x > x {
                galaxy.x += 999999
            }
        })
    });
}

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn expand_space_test() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
            .to_string();
        let space = parse_space(input);
        let mut galaxies = get_galaxies_from_space(&space);
        assert_eq!(
            galaxies,
            vec![
                Galaxy { y: 0, x: 3 },
                Galaxy { y: 1, x: 7 },
                Galaxy { y: 2, x: 0 },
                Galaxy { y: 4, x: 6 },
                Galaxy { y: 5, x: 1 },
                Galaxy { y: 6, x: 9 },
                Galaxy { y: 8, x: 7 },
                Galaxy { y: 9, x: 0 },
                Galaxy { y: 9, x: 4 },
            ]
        );
        expand_space(&space, &mut galaxies);
        assert_eq!(
            galaxies,
            vec![
                Galaxy { y: 0, x: 1000002 },
                Galaxy { y: 1, x: 2000005 },
                Galaxy { y: 2, x: 0 },
                Galaxy { y: 1000003, x: 2000004 },
                Galaxy { y: 1000004, x: 1 },
                Galaxy { y: 1000005, x: 3000006 },
                Galaxy { y: 2000006, x: 2000005 },
                Galaxy { y: 2000007, x: 0 },
                Galaxy { y: 2000007, x: 1000003 },
            ]
        );
    }

    #[test]
    fn solvetest() {
        assert_eq!(82000210, solve(read_test_data_for_day("11-0").unwrap()));
    }
}
