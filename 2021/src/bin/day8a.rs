use std::cmp::Ordering;
use aoc2021::util::read_file_line_by_line_to_string;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("2021/data/8.txt"))
    );
}

fn solve_part_one(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| line.split(" | ").collect::<Vec<&str>>())
        .map(|split| split[1])
        .map(|str| str.to_string())
        .map(|part_after_delimiter| {
            part_after_delimiter
                .split_whitespace()
                .filter(seven_segment_display_is_one_four_seven_eight)
                .collect::<Vec<&str>>()
                .len()
        })
        .sum()
}

fn seven_segment_display_is_one_four_seven_eight(str: &&str) -> bool {
    let length = str.len();
    length == 2 || length == 4 || length == 3 || length == 7
}

#[derive(Debug)]
struct Signal {
    segment: String,
    number: usize,
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Signal {}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.segment == other.segment
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")
        ]
    }

    #[test]
    fn one() {
        assert_eq!(26, solve_part_one(test_data()));
    }
}
