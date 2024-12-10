use aoc2021::util::{comma_separated_string_of_numbers_to_u8_vec, read_data_for_day};
use itertools::Itertools;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_data_for_day("6").unwrap())
    );
}

fn solve_part_one(input: String) -> usize {
    let mut ocean = parse_input(input);
    for i in 0..80 {
        ocean.next_day();
    }
    ocean.size()
}

fn parse_input(input: String) -> Ocean {
    let mut ocean = Ocean::create();
    comma_separated_string_of_numbers_to_u8_vec(&input)
        .into_iter()
        .sorted()
        .dedup_with_count()
        .for_each(|(size, days_old)| ocean.set_fish(days_old, size));
    ocean
}

#[derive(Debug, Eq)]
struct Ocean {
    new_fish: usize,
    amount_of_fish_zero_days_old: usize,
    amount_of_fish_one_days_old: usize,
    amount_of_fish_two_days_old: usize,
    amount_of_fish_three_days_old: usize,
    amount_of_fish_four_days_old: usize,
    amount_of_fish_five_days_old: usize,
    amount_of_fish_six_days_old: usize,
    amount_of_fish_seven_days_old: usize,
    amount_of_fish_eight_days_old: usize,
}

impl PartialEq for Ocean {
    fn eq(&self, other: &Self) -> bool {
        self.amount_of_fish_zero_days_old == other.amount_of_fish_zero_days_old
            && self.amount_of_fish_one_days_old == other.amount_of_fish_one_days_old
            && self.amount_of_fish_two_days_old == other.amount_of_fish_two_days_old
            && self.amount_of_fish_four_days_old == other.amount_of_fish_four_days_old
            && self.amount_of_fish_five_days_old == other.amount_of_fish_five_days_old
            && self.amount_of_fish_six_days_old == other.amount_of_fish_six_days_old
            && self.amount_of_fish_seven_days_old == other.amount_of_fish_seven_days_old
            && self.amount_of_fish_eight_days_old == other.amount_of_fish_eight_days_old
    }
}

impl Ocean {
    fn create() -> Ocean {
        Ocean {
            new_fish: 0,
            amount_of_fish_zero_days_old: 0,
            amount_of_fish_one_days_old: 0,
            amount_of_fish_two_days_old: 0,
            amount_of_fish_three_days_old: 0,
            amount_of_fish_four_days_old: 0,
            amount_of_fish_five_days_old: 0,
            amount_of_fish_six_days_old: 0,
            amount_of_fish_seven_days_old: 0,
            amount_of_fish_eight_days_old: 0,
        }
    }

    fn set_fish(&mut self, days_old: u8, size: usize) {
        match days_old {
            0 => self.amount_of_fish_zero_days_old += size,
            1 => self.amount_of_fish_one_days_old += size,
            2 => self.amount_of_fish_two_days_old += size,
            3 => self.amount_of_fish_three_days_old += size,
            4 => self.amount_of_fish_four_days_old += size,
            5 => self.amount_of_fish_five_days_old += size,
            6 => self.amount_of_fish_six_days_old += size,
            7 => self.amount_of_fish_seven_days_old += size,
            8 => self.amount_of_fish_eight_days_old += size,
            _ => panic!("days_old_in_invalid!"),
        }
    }

    fn next_day(&mut self) {
        self.amount_of_fish_zero_days_old = self.amount_of_fish_one_days_old;
        self.amount_of_fish_one_days_old = self.amount_of_fish_two_days_old;
        self.amount_of_fish_two_days_old = self.amount_of_fish_three_days_old;
        self.amount_of_fish_three_days_old = self.amount_of_fish_four_days_old;
        self.amount_of_fish_four_days_old = self.amount_of_fish_five_days_old;
        self.amount_of_fish_five_days_old = self.amount_of_fish_six_days_old;
        self.amount_of_fish_six_days_old = self.amount_of_fish_seven_days_old + self.new_fish;
        self.amount_of_fish_seven_days_old = self.amount_of_fish_eight_days_old;
        self.amount_of_fish_eight_days_old = self.new_fish;
        self.new_fish = self.amount_of_fish_zero_days_old;
    }

    fn size(&self) -> usize {
        self.amount_of_fish_zero_days_old
            + self.amount_of_fish_one_days_old
            + self.amount_of_fish_two_days_old
            + self.amount_of_fish_three_days_old
            + self.amount_of_fish_four_days_old
            + self.amount_of_fish_five_days_old
            + self.amount_of_fish_six_days_old
            + self.amount_of_fish_seven_days_old
            + self.amount_of_fish_eight_days_old
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_test_data_for_day;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(5934, solve_part_one(read_test_data_for_day("6-0").unwrap()));
    }

    #[test]
    fn parse_input_test() {
        assert_eq!(
            Ocean {
                amount_of_fish_zero_days_old: 0,
                amount_of_fish_one_days_old: 1,
                amount_of_fish_two_days_old: 1,
                amount_of_fish_three_days_old: 2,
                amount_of_fish_four_days_old: 1,
                amount_of_fish_five_days_old: 0,
                amount_of_fish_six_days_old: 0,
                amount_of_fish_seven_days_old: 0,
                amount_of_fish_eight_days_old: 0,
                new_fish: 0
            },
            parse_input(read_test_data_for_day("6-0").unwrap())
        );
    }

    #[test]
    fn next_day_test() {
        let mut initial_state = parse_input(String::from("3,4,3,1,2"));
        assert_eq!(parse_input(String::from("3,4,3,1,2")), initial_state);
        initial_state.next_day();
        assert_eq!(parse_input(String::from("2,3,2,0,1")), initial_state);
        initial_state.next_day();
        assert_eq!(parse_input(String::from("1,2,1,6,0,8")), initial_state);
        initial_state.next_day();
        assert_eq!(parse_input(String::from("0,1,0,5,6,7,8")), initial_state);
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("6,0,6,4,5,6,7,8,8")),
            initial_state
        );
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("5,6,5,3,4,5,6,7,7,8")),
            initial_state
        );
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("4,5,4,2,3,4,5,6,6,7")),
            initial_state
        );
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("3,4,3,1,2,3,4,5,5,6")),
            initial_state
        );
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("2,3,2,0,1,2,3,4,4,5")),
            initial_state
        );
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("1,2,1,6,0,1,2,3,3,4,8")),
            initial_state
        );
        initial_state.next_day();
        assert_eq!(
            parse_input(String::from("0,1,0,5,6,0,1,2,2,3,7,8")),
            initial_state
        );
    }
}