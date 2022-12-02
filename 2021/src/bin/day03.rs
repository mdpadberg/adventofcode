use aoc2021::util::{flip_lines_vec_to_column_vec, read_file_line_by_line_to_string, amount_of_chars_in_string_and_sum_all_chars_to_u32};
use measure_time::print_time;

fn main() {
    print_time!("execution");
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("2021/data/3.txt"))
    );
    println!(
        "solve_part_one -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("2021/data/3.txt"))
    );
    //cargo run --bin day03 --release
    //execution took 0.57ms
}

fn solve_part_one(data: Vec<String>) -> usize {
    let mut gamma: String = String::from("");
    let mut epsilon: String = String::from("");
    let columns: Vec<String> = flip_lines_vec_to_column_vec(&data);
    for column in columns {
        let (amount_of_chars, sum) = amount_of_chars_in_string_and_sum_all_chars_to_u32(&column);
        if sum * 2 < amount_of_chars {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

fn solve_part_two(data: Vec<String>) -> usize {
    usize::from_str_radix(&calculate_oxygen_generator(&data), 2).unwrap()
        * usize::from_str_radix(&calculate_co2_generator(&data), 2).unwrap()
}

fn calculate_oxygen_generator(data: &Vec<String>) -> String {
    let mut oxygen: Vec<String> = data.clone();
    let mut counter: usize = 0;
    while oxygen.len() != 1 {
        let columns: Vec<String> = flip_lines_vec_to_column_vec(&oxygen);
        let (amount_of_chars, sum) = amount_of_chars_in_string_and_sum_all_chars_to_u32(&columns[counter]);
        if sum * 2 >= amount_of_chars {
            oxygen.retain(|col| col.chars().nth(counter).unwrap() == '1')
        } else {
            oxygen.retain(|col| col.chars().nth(counter).unwrap() == '0')
        }
        counter += 1;
    }
    oxygen[0].clone()
}

fn calculate_co2_generator(data: &Vec<String>) -> String {
    let mut co2: Vec<String> = data.clone();
    let mut counter: usize = 0;
    while co2.len() != 1 {
        let columns: Vec<String> = flip_lines_vec_to_column_vec(&co2);
        let (amount_of_chars, sum) = amount_of_chars_in_string_and_sum_all_chars_to_u32(&columns[counter]);
        if sum * 2 >= amount_of_chars {
            co2.retain(|col| col.chars().nth(counter).unwrap() == '0')
        } else {
            co2.retain(|col| col.chars().nth(counter).unwrap() == '1')
        }
        counter += 1;
    }
    co2[0].clone()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ]
    }

    #[test]
    fn one() {
        assert_eq!(198, solve_part_one(test_data()));
    }

    #[test]
    fn two() {
        assert_eq!(230, solve_part_two(test_data()));
    }

    #[test]
    fn oxygen() {
        assert_eq!("10111", calculate_oxygen_generator(&test_data()));
    }

    #[test]
    fn co2() {
        assert_eq!("01010", calculate_co2_generator(&test_data()));
    }
}
