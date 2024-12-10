use std::cmp::Ordering;
use aoc2021::util::{comma_separated_string_of_numbers_to_u32_vec, read_data_for_day};

fn main() {
    println!(
        "solve_part_two -> {:#?}",
        solve_part_two(read_data_for_day("7").unwrap())
    );
}

// mean is 4,9 but dont want to work with floats, so just checking around the mean
fn solve_part_two(input: String) -> u32 {
    let crabs = comma_separated_string_of_numbers_to_u32_vec(&input);
    let mean = get_mean(&crabs);
    let mut fuelpermean = Vec::<FuelPerMean>::new();
    for m in mean - 1..=mean + 1 {
        let mut fuel_sum_per_mean = 0;
        for currentpositioncrab in &crabs {
            fuel_sum_per_mean +=
                calculate_fuel_part_two(calculate_distance(currentpositioncrab, &m));
        }
        fuelpermean.push(FuelPerMean {
            mean: m,
            fuel: fuel_sum_per_mean,
        });
    }
    fuelpermean.into_iter().min().unwrap().fuel
}

fn calculate_distance(from: &u32, to: &u32) -> u32 {
    if from < to {
        return to - from;
    }
    return from - to;
}

fn get_mean(data: &Vec<u32>) -> u32 {
    let length: usize = data.len();
    let sum: u32 = data.into_iter().sum();

    sum / length as u32
}

fn calculate_fuel_part_two(distance: u32) -> u32 {
    if distance <= 0 {
        return 0;
    }
    return distance + calculate_fuel_part_two(distance - 1);
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
    fn two() {
        assert_eq!(168, solve_part_two(read_test_data_for_day("7-0").unwrap()));
    }

    #[test]
    fn test() {
        assert_eq!(66, calculate_fuel_part_two(11));
    }
}
