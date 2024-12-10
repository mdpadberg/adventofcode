use std::cmp::Ordering;
use aoc2021::util::{comma_separated_string_of_numbers_to_u32_vec, read_data_for_day};

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_data_for_day("7").unwrap())
    );
}

fn solve_part_one(input: String) -> u32 {
    let mut data = comma_separated_string_of_numbers_to_u32_vec(&input);
    data.sort();
    let median = get_median(&data);
    data.into_iter()
        .map(|current_position_of_crab| calculate_distance(&current_position_of_crab, &median))
        .sum()
}

fn get_median(data: &Vec<u32>) -> u32 {
    data.get(data.len() / 2).unwrap().clone()
}

fn calculate_distance(from: &u32, to: &u32) -> u32 {
    if from < to {
        return to - from;
    }
    return from - to;
}

#[derive(Debug)]
struct FuelPerMean {
    mean: u32,
    fuel: u32,
}

impl Ord for FuelPerMean {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fuel.cmp(&other.fuel)
    }
}

impl PartialOrd for FuelPerMean {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FuelPerMean {
    fn eq(&self, other: &Self) -> bool {
        self.mean == other.mean && self.fuel == other.fuel
    }
}

impl Eq for FuelPerMean {}

#[cfg(test)]
mod test {
    use aoc2021::util::read_test_data_for_day;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(37, solve_part_one(read_test_data_for_day("7-0").unwrap()));
    }
}
