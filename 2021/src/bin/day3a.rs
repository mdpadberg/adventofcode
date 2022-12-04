use aoc2021::util::{flip_lines_vec_to_column_vec, read_file_line_by_line_to_string, amount_of_chars_in_string_and_sum_all_chars_to_u32};

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("2021/data/3.txt"))
    );
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
}
