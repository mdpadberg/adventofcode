use std::cmp::Ordering;

use aoc2021::util::{
    comma_separated_string_of_numbers_to_u32_vec, comma_separated_strings_of_numbers_to_u32_vec,
    read_file_line_by_line_to_string, replace_space_in_strings_with_comma,
    split_slice_of_string_by_empty_line,
};
use itertools::Itertools;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("4"))
    );
}

//Loop 2x to parse all columns and rows and map to BingoBoard
//Loop 1x to get the highest number of BingoBoard in each row
//Loop 1x to get the highest number of BingoBoard in each column
fn solve_part_one(data: Vec<String>) -> u32 {
    let bingonumbers = parse_bingonumbers(&data);
    let (fastest_bingo_number_for_fastest_bingo_board, fastest_bingo_board) =
        parse_bingoboards(&data, &bingonumbers)
            .into_iter()
            .map(|bingoboard| (bingoboard.calculate_bingo_number(), bingoboard))
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .nth(0)
            .unwrap();
    fastest_bingo_board.bingoboards_score_after_bingo_test(
        &bingonumbers,
        &fastest_bingo_number_for_fastest_bingo_board,
    )
}

fn parse_bingonumbers(data: &Vec<String>) -> BingoNumbers {
    BingoNumbers {
        numbers: comma_separated_string_of_numbers_to_u32_vec(&data[0]),
    }
}

fn parse_bingoboards(data: &Vec<String>, bingonumbers: &BingoNumbers) -> Vec<BingoBoard> {
    split_slice_of_string_by_empty_line(&data[2..data.len()])
        .iter()
        .map(|strings| replace_space_in_strings_with_comma(strings))
        .map(|comma_separated_string| {
            comma_separated_strings_of_numbers_to_u32_vec(&comma_separated_string)
        })
        .map(|numbers| map_bingoboard(numbers, bingonumbers))
        .collect::<Vec<BingoBoard>>()
}

fn map_bingoboard(numbers: Vec<Vec<u32>>, bingonumbers: &BingoNumbers) -> BingoBoard {
    let rows = numbers
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|number| Number {
                    number: number,
                    index_inside_bingonumbers: bingonumbers
                        .numbers
                        .iter()
                        .position(|&b| &b == &number)
                        .unwrap_or(999) as u32,
                })
                .collect()
        })
        .collect();
    let columns = flip_vec_of_vec(&rows);
    BingoBoard {
        rows: rows,
        columns: columns,
    }
}

//make this function from util with generics
fn flip_vec_of_vec(grid: &Vec<Vec<Number>>) -> Vec<Vec<Number>> {
    let mut new_rows = vec![
        vec![
            Number {
                number: 999,
                index_inside_bingonumbers: 999
            };
            grid.len()
        ];
        grid[0].len()
    ];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            new_rows[j][i] = grid[i][j];
        }
    }
    new_rows
}

#[derive(Debug, PartialEq, Clone)]
struct BingoNumbers {
    numbers: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
struct Number {
    number: u32,
    index_inside_bingonumbers: u32,
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index_inside_bingonumbers
            .cmp(&other.index_inside_bingonumbers)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.index_inside_bingonumbers == other.index_inside_bingonumbers
    }
}

impl Eq for Number {}

#[derive(Debug, PartialEq, Clone)]
struct BingoBoard {
    rows: Vec<Vec<Number>>,
    columns: Vec<Vec<Number>>,
}

impl BingoBoard {
    fn calculate_bingo_number(&self) -> Number {
        let bingo_number_per_row = self
            .rows
            .iter()
            .map(|row| row.iter().max())
            .filter(|highest_number_of_row| highest_number_of_row.is_some())
            .map(|highest_number_of_row| highest_number_of_row.unwrap())
            .map(|n| *n)
            .collect::<Vec<Number>>();
        let bingo_number_per_column = self
            .columns
            .iter()
            .map(|column| column.iter().max())
            .filter(|highest_number_of_column| highest_number_of_column.is_some())
            .map(|highest_number_of_column| highest_number_of_column.unwrap())
            .map(|n| *n)
            .collect::<Vec<Number>>();
        [bingo_number_per_row, bingo_number_per_column]
            .concat()
            .into_iter()
            .min()
            .unwrap()
    }

    fn bingoboards_score_after_bingo_test(
        &self,
        bingonumbers: &BingoNumbers,
        bingonumber: &Number,
    ) -> u32 {
        let mut bingonumbers_until_bingo: Vec<u32> = bingonumbers
            .numbers
            .iter()
            .take_while(|number| number != &&bingonumber.number)
            .cloned()
            .collect();
        bingonumbers_until_bingo.push(bingonumber.number);
        let unmarked_numbers: u32 = self
            .rows
            .iter()
            .flatten()
            .filter(|number| !bingonumbers_until_bingo.contains(&number.number))
            .map(|number| number.number)
            .sum();
        unmarked_numbers * bingonumber.number
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(4512, solve_part_one(read_file_line_by_line_to_string_test("4-0")));
    }

    #[test]
    fn parse_bingonumbers_test() {
        let expected_output = BingoNumbers {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
        };
        assert_eq!(expected_output, parse_bingonumbers(&read_file_line_by_line_to_string_test("4-0")));
    }

    #[test]
    fn parse_bingoboards_test() {
        let bingonumbers = BingoNumbers {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
        };
        let input = vec![
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("22 13 17 11  0"),
            String::from("8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from("6 10  3 18  5"),
            String::from("1 12 20 15 19"),
        ];
        let expected_output = vec![BingoBoard {
            rows: vec![
                vec![
                    Number {
                        number: 22,
                        index_inside_bingonumbers: 19,
                    },
                    Number {
                        number: 13,
                        index_inside_bingonumbers: 14,
                    },
                    Number {
                        number: 17,
                        index_inside_bingonumbers: 5,
                    },
                    Number {
                        number: 11,
                        index_inside_bingonumbers: 4,
                    },
                    Number {
                        number: 0,
                        index_inside_bingonumbers: 8,
                    },
                ],
                vec![
                    Number {
                        number: 8,
                        index_inside_bingonumbers: 22,
                    },
                    Number {
                        number: 2,
                        index_inside_bingonumbers: 7,
                    },
                    Number {
                        number: 23,
                        index_inside_bingonumbers: 6,
                    },
                    Number {
                        number: 4,
                        index_inside_bingonumbers: 1,
                    },
                    Number {
                        number: 24,
                        index_inside_bingonumbers: 11,
                    },
                ],
                vec![
                    Number {
                        number: 21,
                        index_inside_bingonumbers: 10,
                    },
                    Number {
                        number: 9,
                        index_inside_bingonumbers: 2,
                    },
                    Number {
                        number: 14,
                        index_inside_bingonumbers: 9,
                    },
                    Number {
                        number: 16,
                        index_inside_bingonumbers: 13,
                    },
                    Number {
                        number: 7,
                        index_inside_bingonumbers: 0,
                    },
                ],
                vec![
                    Number {
                        number: 6,
                        index_inside_bingonumbers: 15,
                    },
                    Number {
                        number: 10,
                        index_inside_bingonumbers: 12,
                    },
                    Number {
                        number: 3,
                        index_inside_bingonumbers: 24,
                    },
                    Number {
                        number: 18,
                        index_inside_bingonumbers: 20,
                    },
                    Number {
                        number: 5,
                        index_inside_bingonumbers: 3,
                    },
                ],
                vec![
                    Number {
                        number: 1,
                        index_inside_bingonumbers: 26,
                    },
                    Number {
                        number: 12,
                        index_inside_bingonumbers: 18,
                    },
                    Number {
                        number: 20,
                        index_inside_bingonumbers: 21,
                    },
                    Number {
                        number: 15,
                        index_inside_bingonumbers: 16,
                    },
                    Number {
                        number: 19,
                        index_inside_bingonumbers: 23,
                    },
                ],
            ],
            columns: vec![
                vec![
                    Number {
                        number: 22,
                        index_inside_bingonumbers: 19,
                    },
                    Number {
                        number: 8,
                        index_inside_bingonumbers: 22,
                    },
                    Number {
                        number: 21,
                        index_inside_bingonumbers: 10,
                    },
                    Number {
                        number: 6,
                        index_inside_bingonumbers: 15,
                    },
                    Number {
                        number: 1,
                        index_inside_bingonumbers: 26,
                    },
                ],
                vec![
                    Number {
                        number: 13,
                        index_inside_bingonumbers: 14,
                    },
                    Number {
                        number: 2,
                        index_inside_bingonumbers: 7,
                    },
                    Number {
                        number: 9,
                        index_inside_bingonumbers: 2,
                    },
                    Number {
                        number: 10,
                        index_inside_bingonumbers: 12,
                    },
                    Number {
                        number: 12,
                        index_inside_bingonumbers: 18,
                    },
                ],
                vec![
                    Number {
                        number: 17,
                        index_inside_bingonumbers: 5,
                    },
                    Number {
                        number: 23,
                        index_inside_bingonumbers: 6,
                    },
                    Number {
                        number: 14,
                        index_inside_bingonumbers: 9,
                    },
                    Number {
                        number: 3,
                        index_inside_bingonumbers: 24,
                    },
                    Number {
                        number: 20,
                        index_inside_bingonumbers: 21,
                    },
                ],
                vec![
                    Number {
                        number: 11,
                        index_inside_bingonumbers: 4,
                    },
                    Number {
                        number: 4,
                        index_inside_bingonumbers: 1,
                    },
                    Number {
                        number: 16,
                        index_inside_bingonumbers: 13,
                    },
                    Number {
                        number: 18,
                        index_inside_bingonumbers: 20,
                    },
                    Number {
                        number: 15,
                        index_inside_bingonumbers: 16,
                    },
                ],
                vec![
                    Number {
                        number: 0,
                        index_inside_bingonumbers: 8,
                    },
                    Number {
                        number: 24,
                        index_inside_bingonumbers: 11,
                    },
                    Number {
                        number: 7,
                        index_inside_bingonumbers: 0,
                    },
                    Number {
                        number: 5,
                        index_inside_bingonumbers: 3,
                    },
                    Number {
                        number: 19,
                        index_inside_bingonumbers: 23,
                    },
                ],
            ],
        }];
        assert_eq!(expected_output, parse_bingoboards(&input, &bingonumbers));
    }

    #[test]
    fn bingoboards_caluclate_bingo_test() {
        let bingonumbers = BingoNumbers {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
        };
        let input = vec![
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from("2  0 12  3  7"),
        ];
        assert_eq!(
            24,
            parse_bingoboards(&input, &bingonumbers)[0]
                .calculate_bingo_number()
                .number
        );
    }

    #[test]
    fn bingoboards_score_after_bingo_test() {
        let bingonumbers = BingoNumbers {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
        };
        let input = vec![
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from("2  0 12  3  7"),
        ];
        let bingonumber = Number {
            number: 24,
            index_inside_bingonumbers: 11,
        };
        assert_eq!(
            4512,
            parse_bingoboards(&input, &bingonumbers)[0]
                .bingoboards_score_after_bingo_test(&bingonumbers, &bingonumber)
        );
    }
}
