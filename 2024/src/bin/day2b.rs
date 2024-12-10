use aoc2024::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("2").unwrap()));
}

fn solve(input: String) -> usize {
    input
        .split("\n")
        .map(|line| {
            is_report_safe_with_one_removed(line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect_vec())
        })
        .filter(|value| *value)
        .count()
}

fn is_report_safe_with_one_removed(input: Vec<i32>) -> bool {
    if is_report_safe(&input) {
        return true;
    }
    for i in 0..input.len() {
        let mut problem_dampened_report = input.clone();
        problem_dampened_report.remove(i);
        if is_report_safe(&problem_dampened_report) {
            return true;
        }
    }
    false
}

fn is_report_safe(input: &Vec<i32>) -> bool {
    let mut output = true;
    for (first, second) in input.iter().tuple_windows() {
        let difference = if input.first().unwrap() - input.last().unwrap() > 0 {
            first - second
        } else {
            second - first
        };
        match difference {
            1..=3 => continue,
            _ => {
                output = false;
                break;
            }
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(4, solve(read_test_data_for_day("2-0").unwrap()));
    }
}
