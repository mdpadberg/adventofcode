use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("6").unwrap()));
}

fn solve(input: String) -> u64 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let data_per_race = lines[0]
        .split(":")
        .zip(lines[1].split(":"))
        .skip(1)
        .map(|(time, distance)| {
            (
                time.chars()
                    .filter(|char| !char.is_whitespace())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
                distance
                    .chars()
                    .filter(|char| !char.is_whitespace())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();
    let mut number_of_ways_you_can_beat_the_record_per_race = vec![0u64; data_per_race.len()];
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
        assert_eq!(71503, solve(read_test_data_for_day("6-0").unwrap()));
    }
}
