use aoc2021::util::read_file_line_by_line_to_string;
use itertools::Itertools;
use measure_time::print_time;

fn main() {
    print_time!("execution");
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("2021/data/10.txt"))
    );
    println!(
        "solve_part_one -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("2021/data/10.txt"))
    );
    //cargo run --bin day10 --release
    //execution took 0.17ms
}

fn solve_part_one(input: Vec<String>) -> u32 {
    parse_intput(input)
        .into_iter()
        .map(|line| NavigationSubSystemLine::create(line))
        .map(|navigationline| navigationline.syntax_error_score())
        .filter(|option| option.is_some())
        .map(|option| option.unwrap())
        .sum()
}

fn solve_part_two(input: Vec<String>) -> usize {
    let incomplete = parse_intput(input)
        .into_iter()
        .map(|line| NavigationSubSystemLine::create(line))
        .filter(|navigationline| navigationline.syntax_error_score().is_none())
        .map(|navigationline| navigationline.fix_incomplete_score())
        .sorted()
        .collect::<Vec<usize>>();
    incomplete[incomplete.len() / 2]
}

fn parse_intput(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

struct NavigationSubSystemLine {
    data: Vec<char>,
}

impl NavigationSubSystemLine {
    fn create(data: Vec<char>) -> NavigationSubSystemLine {
        NavigationSubSystemLine { data }
    }

    fn syntax_error_score(&self) -> Option<u32> {
        let mut signs = Vec::<char>::new();
        for sign in &self.data {
            match sign {
                '(' | '[' | '{' | '<' => signs.push(*sign),
                ')' => {
                    if signs.pop().unwrap() != '(' {
                        return Some(3);
                    }
                }
                ']' => {
                    if signs.pop().unwrap() != '[' {
                        return Some(57);
                    }
                }
                '}' => {
                    if signs.pop().unwrap() != '{' {
                        return Some(1197);
                    }
                }
                '>' => {
                    if signs.pop().unwrap() != '<' {
                        return Some(25137);
                    }
                }
                _ => panic!("unknown sign!"),
            }
        }
        None
    }

    fn fix_incomplete_score(&self) -> usize {
        let mut signs = Vec::<char>::new();
        let mut score = 0;
        for sign in &self.data {
            match sign {
                ')' | ']' | '}' | '>' => {
                    if sign == signs.last().unwrap() {
                        signs.pop();
                    }
                },
                '(' => {
                    signs.push(')');
                }
                '[' => {
                    signs.push(']');
                }
                '{' => {
                    signs.push('}');
                }
                '<' => {
                    signs.push('>');
                }
                _ => panic!("unknown sign!"),
            }
        }
        for sign in signs.iter().rev() {
            score *= 5;
            match sign {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => panic!("not a closing sign!"),
            }
        }
        score
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(26397, solve_part_one(test_data()));
    }

    #[test]
    fn two() {
        assert_eq!(288957, solve_part_two(test_data()));
    }

    #[test]
    fn parse_intput_test() {
        assert_eq!(
            vec![
                vec![
                    '[', '(', '{', '(', '<', '(', '(', ')', ')', '[', ']', '>', '[', '[', '{', '[',
                    ']', '{', '<', '(', ')', '<', '>', '>'
                ],
                vec![
                    '[', '(', '(', ')', '[', '<', '>', ']', ')', ']', '(', '{', '[', '<', '{', '<',
                    '<', '[', ']', '>', '>', '('
                ],
                vec![
                    '{', '(', '[', '(', '<', '{', '}', '[', '<', '>', '[', ']', '}', '>', '{', '[',
                    ']', '{', '[', '(', '<', '(', ')', '>'
                ],
                vec![
                    '(', '(', '(', '(', '{', '<', '>', '}', '<', '{', '<', '{', '<', '>', '}', '{',
                    '[', ']', '{', '[', ']', '{', '}'
                ],
                vec![
                    '[', '[', '<', '[', '(', '[', ']', ')', ')', '<', '(', '[', '[', '{', '}', '[',
                    '[', '(', ')', ']', ']', ']'
                ],
                vec![
                    '[', '{', '[', '{', '(', '{', '}', ']', '{', '}', '}', '(', '[', '{', '[', '{',
                    '{', '{', '}', '}', '(', '[', ']'
                ],
                vec![
                    '{', '<', '[', '[', ']', ']', '>', '}', '<', '{', '[', '{', '[', '{', '[', ']',
                    '{', '(', ')', '[', '[', '[', ']'
                ],
                vec![
                    '[', '<', '(', '<', '(', '<', '(', '<', '{', '}', ')', ')', '>', '<', '(', '[',
                    ']', '(', '[', ']', '(', ')'
                ],
                vec![
                    '<', '{', '(', '[', '(', '[', '[', '(', '<', '>', '(', ')', ')', '{', '}', ']',
                    '>', '(', '<', '<', '{', '{'
                ],
                vec![
                    '<', '{', '(', '[', '{', '{', '}', '}', '[', '<', '[', '[', '[', '<', '>', '{',
                    '}', ']', ']', ']', '>', '[', ']', ']'
                ]
            ],
            parse_intput(test_data())
        )
    }

    #[test]
    fn syntax_error_score_test() {
        assert_eq!(
            Some(1197),
            NavigationSubSystemLine {
                data: vec![
                    '{', '(', '[', '(', '<', '{', '}', '[', '<', '>', '[', ']', '}', '>', '{', '[',
                    ']', '{', '[', '(', '<', '(', ')', '>'
                ]
            }
            .syntax_error_score()
        );
        assert_eq!(
            Some(3),
            NavigationSubSystemLine {
                data: vec![
                    '[', '[', '<', '[', '(', '[', ']', ')', ')', '<', '(', '[', '[', '{', '}', '[',
                    '[', '(', ')', ']', ']', ']'
                ]
            }
            .syntax_error_score()
        );
        assert_eq!(
            Some(3),
            NavigationSubSystemLine {
                data: vec![
                    '[', '[', '<', '[', '(', '[', ']', ')', ')', '<', '(', '[', '[', '{', '}', '[',
                    '[', '(', ')', ']', ']', ']'
                ]
            }
            .syntax_error_score()
        );
        assert_eq!(
            Some(57),
            NavigationSubSystemLine {
                data: vec![
                    '[', '{', '[', '{', '(', '{', '}', ']', '{', '}', '}', '(', '[', '{', '[', '{',
                    '{', '{', '}', '}', '(', '[', ']'
                ]
            }
            .syntax_error_score()
        );
        assert_eq!(
            Some(3),
            NavigationSubSystemLine {
                data: vec![
                    '[', '<', '(', '<', '(', '<', '(', '<', '{', '}', ')', ')', '>', '<', '(', '[',
                    ']', '(', '[', ']', '(', ')'
                ]
            }
            .syntax_error_score()
        );
        assert_eq!(
            Some(25137),
            NavigationSubSystemLine {
                data: vec![
                    '<', '{', '(', '[', '(', '[', '[', '(', '<', '>', '(', ')', ')', '{', '}', ']',
                    '>', '(', '<', '<', '{', '{'
                ]
            }
            .syntax_error_score()
        );
        assert_eq!(
            None,
            NavigationSubSystemLine {
                data: vec![
                    '<', '{', '(', '[', '{', '{', '}', '}', '[', '<', '[', '[', '[', '<', '>', '{',
                    '}', ']', ']', ']', '>', '[', ']', ']'
                ]
            }
            .syntax_error_score()
        );
    }

    #[test]
    fn fix_incomplete_score_test() {
        assert_eq!(
            288957,
            NavigationSubSystemLine {
                data: vec![
                    '[', '(', '{', '(', '<', '(', '(', ')', ')', '[', ']', '>', '[', '[', '{', '[',
                    ']', '{', '<', '(', ')', '<', '>', '>'
                ],
            }
            .fix_incomplete_score()
        );
        assert_eq!(
            5566,
            NavigationSubSystemLine {
                data: vec![
                    '[', '(', '(', ')', '[', '<', '>', ']', ')', ']', '(', '{', '[', '<', '{', '<',
                    '<', '[', ']', '>', '>', '('
                ]
            }
            .fix_incomplete_score()
        );
        assert_eq!(
            1480781,
            NavigationSubSystemLine {
                data: vec![
                    '(', '(', '(', '(', '{', '<', '>', '}', '<', '{', '<', '{', '<', '>', '}', '{',
                    '[', ']', '{', '[', ']', '{', '}'
                ]
            }
            .fix_incomplete_score()
        );
        assert_eq!(
            995444,
            NavigationSubSystemLine {
                data: vec![
                    '{', '<', '[', '[', ']', ']', '>', '}', '<', '{', '[', '{', '[', '{', '[', ']',
                    '{', '(', ')', '[', '[', '[', ']'
                ],
            }
            .fix_incomplete_score()
        );
        assert_eq!(
            294,
            NavigationSubSystemLine {
                data: vec![
                    '<', '{', '(', '[', '{', '{', '}', '}', '[', '<', '[', '[', '[', '<', '>', '{',
                    '}', ']', ']', ']', '>', '[', ']', ']'
                ]
            }
            .fix_incomplete_score()
        );
    }

    fn test_data() -> Vec<String> {
        vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ]
    }
}
