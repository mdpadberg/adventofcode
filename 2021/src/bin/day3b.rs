use aoc2021::util::{flip_lines_vec_to_column_vec, read_file_line_by_line_to_string, amount_of_chars_in_string_and_sum_all_chars_to_u32};

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("3"))
    );
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
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;
    
    #[test]
    fn two() {
        assert_eq!(230, solve_part_two(read_file_line_by_line_to_string_test("3-0")));
    }

    #[test]
    fn oxygen() {
        assert_eq!("10111", calculate_oxygen_generator(&read_file_line_by_line_to_string_test("3-0")));
    }

    #[test]
    fn co2() {
        assert_eq!("01010", calculate_co2_generator(&read_file_line_by_line_to_string_test("3-0")));
    }
}
