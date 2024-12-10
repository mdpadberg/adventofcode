use itertools::Itertools;

use aoc2021::util::read_file_line_by_line_to_string;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("5"))
    );
    // Improvement: implement enum so only have to loop once for assignment two
    // struct Line {
    //      from: Dot,
    //      to: Dot,
    //      typeOfLine: TypeOfLine
    // }
    // enum TypeOfLine {
    //     HORIZONTAL,
    //     VERTICAL,
    //     DIAGONAL
    // }
}

fn solve_part_one(data: Vec<String>) -> usize {
    parse_input(&data)
        .into_iter()
        .filter(is_horizontal_or_vertical_line)
        .map(from_horizontal_or_vertical_line_to_dots)
        .flatten()
        .sorted()
        .dedup_with_count()
        .into_iter()
        .filter(|predicate| predicate.0 > 1)
        .map(|f| f.1)
        .collect::<Vec<Dot>>()
        .len()
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"([0-9]+),([0-9]+)\s->\s([0-9]+),([0-9]+)").unwrap();
}

fn parse_input(data: &Vec<String>) -> Vec<Line> {
    data.iter()
        .map(|line| REGEX.captures(line).unwrap())
        .map(|captures| Line {
            from: Dot {
                x: captures[1].parse::<u32>().unwrap(),
                y: captures[2].parse::<u32>().unwrap(),
            },
            to: Dot {
                x: captures[3].parse::<u32>().unwrap(),
                y: captures[4].parse::<u32>().unwrap(),
            },
        })
        .collect::<Vec<Line>>()
}

fn is_horizontal_or_vertical_line(line: &Line) -> bool {
    line.from.x == line.to.x || line.from.y == line.to.y
}

fn from_horizontal_or_vertical_line_to_dots(line: Line) -> Vec<Dot> {
    let mut horizontal_line = loop_x(line.from.x, line.to.x, line.from.y);
    let vertical_line = loop_y(line.from.y, line.to.y, line.from.x);
    horizontal_line.extend(vertical_line);
    horizontal_line
}

fn loop_x(from: u32, to: u32, y: u32) -> Vec<Dot> {
    let mut dots = Vec::new();
    if from != to {
        let reverse = if from > to { true } else { false };
        if reverse {
            for i in to..=from {
                dots.push(Dot { x: i, y: y });
            }
            dots.reverse();
        } else {
            for i in from..=to {
                dots.push(Dot { x: i, y: y });
            }
        }
    }
    dots
}

fn loop_y(from: u32, to: u32, x: u32) -> Vec<Dot> {
    let mut dots = Vec::new();
    if from != to {
        let reverse = if from > to { true } else { false };
        if reverse {
            for i in to..=from {
                dots.push(Dot { x: x, y: i });
            }
            dots.reverse();
        } else {
            for i in from..=to {
                dots.push(Dot { x: x, y: i });
            }
        }
    }
    dots
}

#[derive(Debug, PartialEq)]
struct Line {
    from: Dot,
    to: Dot,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
struct Dot {
    x: u32,
    y: u32,
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(5, solve_part_one(read_file_line_by_line_to_string_test("5-0")));
    }

    #[test]
    fn parser() {
        let input = vec![String::from("0,9 -> 5,9"), String::from("8,0 -> 0,8")];
        let expected_output = vec![
            Line {
                from: Dot { x: 0, y: 9 },
                to: Dot { x: 5, y: 9 },
            },
            Line {
                from: Dot { x: 8, y: 0 },
                to: Dot { x: 0, y: 8 },
            },
        ];
        assert_eq!(expected_output, parse_input(&input));
    }

    #[test]
    fn horizontal_or_vertical_line_filter() {
        assert_eq!(
            true,
            is_horizontal_or_vertical_line(&Line {
                from: Dot { x: 0, y: 9 },
                to: Dot { x: 5, y: 9 }
            })
        );
        assert_eq!(
            true,
            is_horizontal_or_vertical_line(&Line {
                from: Dot { x: 1, y: 1 },
                to: Dot { x: 1, y: 3 }
            })
        );
    }

    #[test]
    fn line_to_dots() {
        assert_eq!(
            vec![
                Dot { x: 0, y: 9 },
                Dot { x: 1, y: 9 },
                Dot { x: 2, y: 9 },
                Dot { x: 3, y: 9 },
                Dot { x: 4, y: 9 },
                Dot { x: 5, y: 9 },
            ],
            from_horizontal_or_vertical_line_to_dots(Line {
                from: Dot { x: 0, y: 9 },
                to: Dot { x: 5, y: 9 },
            })
        );
        assert_eq!(
            vec![Dot { x: 0, y: 9 }, Dot { x: 1, y: 9 }, Dot { x: 2, y: 9 },],
            from_horizontal_or_vertical_line_to_dots(Line {
                from: Dot { x: 0, y: 9 },
                to: Dot { x: 2, y: 9 },
            })
        );
        assert_eq!(
            vec![Dot { x: 1, y: 1 }, Dot { x: 1, y: 2 }, Dot { x: 1, y: 3 },],
            from_horizontal_or_vertical_line_to_dots(Line {
                from: Dot { x: 1, y: 1 },
                to: Dot { x: 1, y: 3 },
            })
        );
        assert_eq!(
            vec![Dot { x: 9, y: 7 }, Dot { x: 8, y: 7 }, Dot { x: 7, y: 7 },],
            from_horizontal_or_vertical_line_to_dots(Line {
                from: Dot { x: 9, y: 7 },
                to: Dot { x: 7, y: 7 },
            })
        );
    }

    #[test]
    fn loop_x_test() {
        let expected_output_one = vec![Dot { x: 0, y: 9 }, Dot { x: 1, y: 9 }, Dot { x: 2, y: 9 }];
        assert_eq!(expected_output_one, loop_x(0, 2, 9));

        let expected_output_two = vec![Dot { x: 9, y: 7 }, Dot { x: 8, y: 7 }, Dot { x: 7, y: 7 }];
        assert_eq!(expected_output_two, loop_x(9, 7, 7));
    }

    #[test]
    fn loop_y_test() {
        let expected_output_one = vec![Dot { x: 0, y: 7 }, Dot { x: 0, y: 8 }, Dot { x: 0, y: 9 }];
        assert_eq!(expected_output_one, loop_y(7, 9, 0));

        let expected_output_two = vec![Dot { x: 1, y: 5 }, Dot { x: 1, y: 4 }, Dot { x: 1, y: 3 }];
        assert_eq!(expected_output_two, loop_y(5, 3, 1));
    }
}
