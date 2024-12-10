use std::cmp::Ordering;

use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use serde_json::Value;

fn main() {
    println!("{}", solve(read_data_for_day("13").unwrap()));
}

fn solve(input: String) -> usize {
    let start = vec![
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    let values = input
        .split("\n\n")
        .flat_map(|p| p.split("\n").collect_tuple())
        .flat_map(|pair_of_packets: (&str, &str)| {
            vec![
                serde_json::from_str::<Value>(pair_of_packets.0).unwrap(),
                serde_json::from_str::<Value>(pair_of_packets.1).unwrap(),
            ]
        })
        .collect::<Vec<Value>>();
    [&start[..], &values[..]]
        .concat()
        .iter()
        .sorted_by(|a, b| {
            if packets_are_in_correct_order(a, b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .enumerate()
        .map(|f| (f.0 + 1, f.1.to_string()))
        .filter(|f| f.1 == "[[2]]" || f.1 == "[[6]]")
        .map(|f| f.0)
        .product()
}

fn packets_are_in_correct_order(left: &Value, right: &Value) -> bool {
    match (
        left.is_u64(),
        right.is_u64(),
        left.is_array(),
        right.is_array(),
    ) {
        (true, true, false, false) => left.as_u64() < right.as_u64(),
        (false, false, true, true) => correct(left.as_array().unwrap(), right.as_array().unwrap()),
        (false, true, true, false) => {
            let temp =
                serde_json::from_str(&format!("{:?}", vec![right.as_u64().unwrap()])).unwrap();
            packets_are_in_correct_order(left, &temp)
        }
        (true, false, false, true) => {
            let temp =
                serde_json::from_str(&format!("{:?}", vec![left.as_u64().unwrap()])).unwrap();
            packets_are_in_correct_order(&temp, right)
        }
        _ => {
            panic!("AAAAAAAAAAAH!");
        }
    }
}

fn correct(left: &Vec<Value>, right: &Vec<Value>) -> bool {
    let mut result = false;
    for values in left.iter().zip_longest(right) {
        match values {
            itertools::EitherOrBoth::Both(l, r) => {
                if l == r {
                    result = true;
                } else {
                    return packets_are_in_correct_order(l, r);
                };
            }
            itertools::EitherOrBoth::Left(_) => return false,
            itertools::EitherOrBoth::Right(_) => return true,
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;
    #[test]
    fn solvetest() {
        assert_eq!(140, solve(read_test_data_for_day("13-0").unwrap()));
    }
}
