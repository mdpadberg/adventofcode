use aoc2022::util::read_data_for_day;

fn main() {
    println!("{}", solve(read_data_for_day("1").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines())
        .map(|inventory| {
            inventory
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(24000, solve(read_test_data_for_day("1-0").unwrap()));
    }
}
