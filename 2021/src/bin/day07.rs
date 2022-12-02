use std::cmp::Ordering;
use aoc2021::util::{comma_separated_string_of_numbers_to_u32_vec, read_file};
use measure_time::print_time;

fn main() {
    print_time!("execution");
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file("2021/data/7.txt"))
    );
    println!(
        "solve_part_two -> {:#?}",
        solve_part_two(read_file("2021/data/7.txt"))
    );
    //cargo run --bin day07 --release
    //execution took 0.06ms
}

fn solve_part_one(input: String) -> u32 {
    let mut data = comma_separated_string_of_numbers_to_u32_vec(&input);
    data.sort();
    let median = get_median(&data);
    data.into_iter()
        .map(|current_position_of_crab| calculate_distance(&current_position_of_crab, &median))
        .sum()
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

fn get_median(data: &Vec<u32>) -> u32 {
    data.get(data.len() / 2).unwrap().clone()
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
    use super::*;

    fn test_data() -> String {
        String::from("16,1,2,0,4,2,7,1,2,14")
    }

    #[test]
    fn one() {
        assert_eq!(37, solve_part_one(test_data()));
    }

    #[test]
    fn two() {
        assert_eq!(168, solve_part_two(test_data()));
    }

    #[test]
    fn test() {
        assert_eq!(66, calculate_fuel_part_two(11));
    }
}
