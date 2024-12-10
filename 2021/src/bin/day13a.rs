use aoc2021::util::read_file_line_by_line_to_string;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("13"))
    );
}

fn solve_part_one(input: Vec<String>) -> usize {
    let mut transparent_paper = TransparentPaper::new(input);
    transparent_paper.fold_once();
    transparent_paper.dots.len()
}

#[derive(Debug, PartialEq, Eq)]
struct TransparentPaper {
    dots: Vec<(u32, u32)>,
    fold: Vec<(char, u32)>,
    columns: u32,
    rows: u32,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"fold along (.)=(\d+)").unwrap();
}

impl TransparentPaper {
    fn new(input: Vec<String>) -> TransparentPaper {
        let dots: Vec<(u32, u32)> = input
            .iter()
            .filter(|line| !line.is_empty() && !line.starts_with("fold"))
            .map(|line| line.split(",").collect())
            .map(|values: Vec<&str>| (values[0].parse().unwrap(), values[1].parse().unwrap()))
            .collect();
        TransparentPaper {
            fold: input
                .iter()
                .filter(|line| line.starts_with("fold"))
                .map(|line| REGEX.captures(line).unwrap())
                .map(|captures| {
                    (
                        captures[1].parse::<char>().unwrap(),
                        captures[2].parse::<u32>().unwrap(),
                    )
                })
                .rev()
                .collect(),
            columns: dots.iter().map(|dot| dot.0).max().unwrap(),
            rows: dots.iter().map(|dot| dot.1).max().unwrap(),
            dots: dots,
        }
    }

    fn fold_all(&mut self) {
        while !self.fold.is_empty() {
            self.fold_once()
        }
    }

    fn fold_once(&mut self) {
        let fold_instruction = self.fold.pop().unwrap();
        let mut new_dots = Vec::<(u32, u32)>::with_capacity(self.dots.len());
        if fold_instruction.0 == 'y' {
            for dot in &self.dots {
                if dot.1 > fold_instruction.1 {
                    let difference = dot.1 - fold_instruction.1;
                    new_dots.push((dot.0, dot.1 - (difference * 2)));
                } else {
                    new_dots.push((dot.0, dot.1));
                }
            }
            self.rows = self.rows / 2;
        } else {
            for dot in &self.dots {
                if dot.0 > fold_instruction.1 {
                    let difference = dot.0 - fold_instruction.1;
                    new_dots.push((dot.0 - (difference * 2), dot.1));
                } else {
                    new_dots.push((dot.0, dot.1));
                }
            }
            self.columns = self.columns / 2;
        }
        new_dots.sort();
        new_dots.dedup();
        self.dots = new_dots;
    }

    fn print_grid(&self) -> Vec<String> {
        let mut grid: Vec<String> = Vec::<String>::with_capacity(self.rows as usize);
        for i in 0..self.rows {
            let mut row = String::from("");
            for j in 0..self.columns {
                if self.dots.contains(&(j, i)) {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            grid.push(row);
        }
        grid
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(17, solve_part_one(read_file_line_by_line_to_string_test("13-1")));
    }

    #[test]
    fn create_transparent_paper() {
        let expected = TransparentPaper {
            dots: vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ],
            fold: vec![('x', 5), ('y', 7)],
            columns: 10,
            rows: 14,
        };
        assert_eq!(expected, TransparentPaper::new(read_file_line_by_line_to_string_test("13-1")));
    }

    #[test]
    fn transparent_paper_fold() {
        let mut transparent_paper = TransparentPaper::new(read_file_line_by_line_to_string_test("13-1"));
        transparent_paper.fold_once();
        assert_eq!(
            vec![
                (0, 0),
                (0, 1),
                (0, 3),
                (1, 4),
                (2, 0),
                (3, 0),
                (3, 4),
                (4, 1),
                (4, 3),
                (6, 0),
                (6, 2),
                (6, 4),
                (8, 4),
                (9, 0),
                (9, 4),
                (10, 2),
                (10, 4),
            ],
            transparent_paper.dots
        );
        transparent_paper.fold_once();
        assert_eq!(
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 0),
                (1, 4),
                (2, 0),
                (2, 4),
                (3, 0),
                (3, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ],
            transparent_paper.dots
        );
    }
}
