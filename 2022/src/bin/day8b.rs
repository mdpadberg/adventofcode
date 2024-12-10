use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("8").unwrap()));
}

fn solve(input: String) -> usize {
    let grid = input
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut scores = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let current_tree = Coordinates { y, x };
            let trees_above = Direction::UP(current_tree.clone()).get(&grid);
            let trees_down = Direction::DOWN(current_tree.clone()).get(&grid);
            let trees_left = Direction::LEFT(current_tree.clone()).get(&grid);
            let trees_right = Direction::RIGHT(current_tree.clone()).get(&grid);
            let amount_of_trees_visable_from_above =
                trees_above.iter().take_while(|v| v < &value).count();
            let amount_of_trees_visable_from_below =
                trees_down.iter().take_while(|v| v < &value).count();
            let amount_of_trees_visable_from_left =
                trees_left.iter().take_while(|v| v < &value).count();
            let amount_of_trees_visable_from_right =
                trees_right.iter().take_while(|v| v < &value).count();
            let score_above = if amount_of_trees_visable_from_above != trees_above.iter().count() {
                amount_of_trees_visable_from_above + 1
            } else {
                amount_of_trees_visable_from_above
            };
            let score_below = if amount_of_trees_visable_from_below != trees_down.iter().count() {
                amount_of_trees_visable_from_below + 1
            } else {
                amount_of_trees_visable_from_below
            };
            let score_left = if amount_of_trees_visable_from_left != trees_left.iter().count() {
                amount_of_trees_visable_from_left + 1
            } else {
                amount_of_trees_visable_from_left
            };
            let score_right = if amount_of_trees_visable_from_right != trees_right.iter().count() {
                amount_of_trees_visable_from_right + 1
            } else {
                amount_of_trees_visable_from_right
            };
            scores.push(score_above * score_below * score_left * score_right);
        }
    }
    *scores.iter().max().unwrap()
}

#[derive(Debug, Clone)]
struct Coordinates {
    y: usize,
    x: usize,
}

#[derive(Debug)]
enum Direction {
    UP(Coordinates),
    DOWN(Coordinates),
    LEFT(Coordinates),
    RIGHT(Coordinates),
}

impl Direction {
    fn get(&self, grid: &Vec<Vec<u32>>) -> Vec<u32> {
        match self {
            Direction::UP(Coordinates { x, y }) => {
                if y == &0 {
                    vec![]
                } else {
                    grid.iter().take(*y).map(|row| row[*x]).rev().collect()
                }
            }
            Direction::DOWN(Coordinates { x, y }) => {
                let max_y: usize = grid.iter().count() - 1;
                if y == &max_y {
                    vec![]
                } else {
                    grid.iter()
                        .rev()
                        .take(max_y - y)
                        .map(|row| row[*x])
                        .rev()
                        .collect()
                }
            }
            Direction::LEFT(Coordinates { x, y }) => {
                if x == &0 {
                    vec![]
                } else {
                    grid[*y].iter().take(*x).rev().map(|v| *v).collect()
                }
            }
            Direction::RIGHT(Coordinates { x, y }) => {
                let max_x: usize = grid[*y].len() - 1;
                if x == &max_x {
                    vec![]
                } else {
                    grid[*y]
                        .iter()
                        .rev()
                        .take(max_x - x)
                        .rev()
                        .map(|v| *v)
                        .collect()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(8, solve(read_test_data_for_day("8-0").unwrap()));
    }

    #[test]
    fn up() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![2, 5, 5, 1, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(
            vec![] as Vec<u32>,
            Direction::UP(Coordinates { x: 0, y: 0 }).get(&grid)
        );
        assert_eq!(
            vec![0],
            Direction::UP(Coordinates { x: 1, y: 1 }).get(&grid)
        );
        assert_eq!(
            vec![5, 3],
            Direction::UP(Coordinates { x: 2, y: 2 }).get(&grid)
        );
        assert_eq!(
            vec![1, 1, 7],
            Direction::UP(Coordinates { x: 3, y: 3 }).get(&grid)
        );
        assert_eq!(
            vec![9, 2, 2, 3],
            Direction::UP(Coordinates { x: 4, y: 4 }).get(&grid)
        );
    }

    #[test]
    fn down() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![2, 5, 5, 1, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(
            vec![] as Vec<u32>,
            Direction::DOWN(Coordinates { x: 4, y: 4 }).get(&grid)
        );
        assert_eq!(
            vec![9],
            Direction::DOWN(Coordinates { x: 3, y: 3 }).get(&grid)
        );
        assert_eq!(
            vec![5, 3],
            Direction::DOWN(Coordinates { x: 2, y: 2 }).get(&grid)
        );
        assert_eq!(
            vec![5, 3, 5],
            Direction::DOWN(Coordinates { x: 1, y: 1 }).get(&grid)
        );
        assert_eq!(
            vec![2, 2, 3, 3],
            Direction::DOWN(Coordinates { x: 0, y: 0 }).get(&grid)
        );
    }

    #[test]
    fn left() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![2, 5, 5, 1, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(
            vec![] as Vec<u32>,
            Direction::LEFT(Coordinates { x: 0, y: 0 }).get(&grid)
        );
        assert_eq!(
            vec![2],
            Direction::LEFT(Coordinates { x: 1, y: 1 }).get(&grid)
        );
        assert_eq!(
            vec![5, 2],
            Direction::LEFT(Coordinates { x: 2, y: 2 }).get(&grid)
        );
        assert_eq!(
            vec![5, 3, 3],
            Direction::LEFT(Coordinates { x: 3, y: 3 }).get(&grid)
        );
        assert_eq!(
            vec![9, 3, 5, 3],
            Direction::LEFT(Coordinates { x: 4, y: 4 }).get(&grid)
        );
    }

    #[test]
    fn right() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![2, 5, 5, 1, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(
            vec![] as Vec<u32>,
            Direction::RIGHT(Coordinates { x: 4, y: 4 }).get(&grid)
        );
        assert_eq!(
            vec![9],
            Direction::RIGHT(Coordinates { x: 3, y: 3 }).get(&grid)
        );
        assert_eq!(
            vec![1, 2],
            Direction::RIGHT(Coordinates { x: 2, y: 2 }).get(&grid)
        );
        assert_eq!(
            vec![5, 1, 2],
            Direction::RIGHT(Coordinates { x: 1, y: 1 }).get(&grid)
        );
        assert_eq!(
            vec![0, 3, 7, 3],
            Direction::RIGHT(Coordinates { x: 0, y: 0 }).get(&grid)
        );
    }
}
