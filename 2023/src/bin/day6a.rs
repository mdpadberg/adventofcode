use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("6").unwrap()));
}

fn solve(input: String) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let data_per_race = lines[0]
        .split_whitespace()
        .zip(lines[1].split_ascii_whitespace())
        .skip(1)
        .map(|(time, distance)| {
            (
                time.parse::<u32>().unwrap(),
                distance.parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();
    let mut number_of_ways_you_can_beat_the_record_per_race = vec![0u32; data_per_race.len()];
    data_per_race
        .iter()
        .enumerate()
        .for_each(|(race_number, (time, distance))| {
            for speed in 0..=time.to_owned() {
                let time_left = time - speed;
                let result = speed * time_left;
                if &result > distance {
                    number_of_ways_you_can_beat_the_record_per_race[race_number] += 1;
                }
            }
        });
    number_of_ways_you_can_beat_the_record_per_race
        .iter()
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(288, solve(read_test_data_for_day("6-0").unwrap()));
    }
}
