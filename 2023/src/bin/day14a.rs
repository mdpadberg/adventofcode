use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("14").unwrap()));
}

fn solve(input: String) -> usize {
    let puzzle_input_vertically = flip_puzzle_input(input);
    let moved_rounded_rocks = puzzle_input_vertically
        .into_iter()
        .map(|line| move_rocks_based_on_tilt(line))
        .join("\n");
    let puzzle_input_horizontally = flip_puzzle_input(moved_rounded_rocks)
        .into_iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>();
    calculate_score(puzzle_input_horizontally)
}

fn flip_puzzle_input(input: String) -> Vec<Vec<char>> {
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
    vertical
}

// Take this data as an example:  
// - vec!['#', '.', '#', '.', '.', 'O', '#', '.', '#', '#']
// What will happen:
// - split based on #                               -> vec![vec!['.'], vec!['.', '.', 'O'], vec!['.'], vec![], vec![]]
// - count the amount of O and the amount of chars  -> (0,1), (1,3), (0,1), (0,), (0,0)
// - fill back the array with O at the beginning    -> vec![vec!['.'], vec!['O', '.', '.'], vec!['.'], vec![], vec![]]
// - merge the vecs with # seperator                -> #.#O..#.##
fn move_rocks_based_on_tilt(values: Vec<char>) -> String {
    values
        .split(|&value| value == '#')
        .map(|chars| {
            (
                chars.len(),
                chars.iter().filter(|char| char == &&'O').count(),
            )
        })
        .map(|(amount_of_chars, amount_of_round_rocks)| {
            let amount_of_empty_spaces = amount_of_chars - amount_of_round_rocks;
            let mut result = vec!['O'; amount_of_round_rocks];
            result.extend(vec!['.'; amount_of_empty_spaces]);
            result
        })
        .map(|chars| chars.iter().collect::<String>())
        .join("#")
}

fn calculate_score(input: Vec<String>) -> usize {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| (i + 1) * line.chars().filter(|&char| char == 'O').count())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn move_rocks_based_on_tilt_test() {
        assert_eq!(
            String::from("OOOO....##"),
            move_rocks_based_on_tilt(vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#'])
        );
        assert_eq!(
            String::from("OOO......."),
            move_rocks_based_on_tilt(vec!['.', '.', '.', 'O', 'O', '.', '.', '.', '.', 'O'])
        );
        assert_eq!(
            String::from("O....#OO.."),
            move_rocks_based_on_tilt(vec!['.', 'O', '.', '.', '.', '#', 'O', '.', '.', 'O'])
        );
        assert_eq!(
            String::from("O..#......"),
            move_rocks_based_on_tilt(vec!['.', 'O', '.', '#', '.', '.', '.', '.', '.', '.'])
        );
        assert_eq!(
            String::from(".#O......."),
            move_rocks_based_on_tilt(vec!['.', '#', '.', 'O', '.', '.', '.', '.', '.', '.'])
        );
        assert_eq!(
            String::from("#.#O..#.##"),
            move_rocks_based_on_tilt(vec!['#', '.', '#', '.', '.', 'O', '#', '.', '#', '#'])
        );
        assert_eq!(
            String::from("..#O....#."),
            move_rocks_based_on_tilt(vec!['.', '.', '#', '.', '.', '.', 'O', '.', '#', '.'])
        );
        assert_eq!(
            String::from("O....#O.#."),
            move_rocks_based_on_tilt(vec!['.', '.', '.', '.', 'O', '#', '.', 'O', '#', '.'])
        );
        assert_eq!(
            String::from("....#....."),
            move_rocks_based_on_tilt(vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'])
        );
        assert_eq!(
            String::from(".#O..#O..."),
            move_rocks_based_on_tilt(vec!['.', '#', '.', 'O', '.', '#', 'O', '.', '.', '.'])
        );
    }

    #[test]
    fn parse_moutain_test() {
        let input = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;
        assert_eq!(
            136,
            calculate_score(
                input
                    .split("\n")
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            )
        );
    }

    #[test]
    fn solvetest() {
        assert_eq!(136, solve(read_test_data_for_day("14-0").unwrap()));
    }
}
