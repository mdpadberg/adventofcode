use aoc2022::util::read_data_for_day;

fn main() {
    println!("{}", solve(read_data_for_day("3").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group_of_elfs| {
            let (elf_one, elf_two, elf_three) =
                (group_of_elfs[0], group_of_elfs[1], group_of_elfs[2]);
            let position_common_item = elf_one
                .find(|item_in_rucksack| elf_two.contains(item_in_rucksack) && elf_three.contains(item_in_rucksack))
                .unwrap();
            elf_one.chars().nth(position_common_item).unwrap()
        })
        .map(|item_in_rucksack| {
            if item_in_rucksack.is_lowercase() {
                item_in_rucksack as u32 - 96
            } else {
                item_in_rucksack as u32 - 38
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use aoc2022::util::read_test_data_for_day;

    use super::*;

    #[test]
    fn solvetest() {
        assert_eq!(70, solve(read_test_data_for_day("3-0").unwrap()));
    }
}
