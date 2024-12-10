use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("20").unwrap()));
}

fn solve(input: String) -> i64 {
    let numbers = input
        .split("\n")
        .map(|str| str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let result = mix(numbers);
    let position_of_zero = result.iter().position(|i| i == &0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|th| (position_of_zero + th) % result.len())
        .map(|i| result[i])
        .sum()
}

/// For example:
/// Initial index:
/// 0, 1, 2, 3, 4, 5, 6
/// Initial values:
/// "1", "2", "-3", "3", "-2", "0", "4"
///
/// Indices after 4 iterations of the example file:
/// 0, 1, 4, 2, 5, 3, 6
/// Values after 4 iterations of the example file:
/// "1", "2", "-2", "-3", "0", "3", "4"
fn mix(values: Vec<i64>) -> Vec<i64> {
    let mut indices = Vec::from_iter(0..values.len());
    for i in 0..values.len() {
        let current_index = indices.iter().position(|p| p == &i).unwrap();
        indices.remove(current_index);
        let new_index = calculate_new_index(current_index, values[i], values.len());
        if new_index == 0 {
            indices.push(i);
        } else {
            indices.insert(new_index as usize, i);
        }
    }
    // Map indices back to the values
    indices.iter().map(|&i| values[i]).collect::<Vec<i64>>()
}

/// % vs rem_euclid -> https://doc.rust-lang.org/std/primitive.i64.html#method.rem_euclid
fn calculate_new_index(
    current_index: usize,
    value_of_current_index: i64,
    length_of_vec: usize,
) -> i64 {
    (current_index as i64 + value_of_current_index).rem_euclid(length_of_vec as i64 - 1)
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn mixtest() {
        assert_eq!(
            vec![1, 2, -3, 4, 0, 3, -2],
            mix(read_test_data_for_day("20-0")
                .unwrap()
                .split("\n")
                .map(|str| str.parse::<i64>().unwrap())
                .collect::<Vec<i64>>())
        );
    }

    #[test]
    fn solvetest() {
        assert_eq!(3, solve(read_test_data_for_day("20-0").unwrap()));
    }
}
