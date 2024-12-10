use std::path::PathBuf;

pub fn read_data_for_day(filename: &str) -> Option<String> {
    read_data(&filename, "data")
}

pub fn read_test_data_for_day(filename: &str) -> Option<String> {
    read_data(&filename, "test-data")
}

fn read_data(filename: &str, folder: &str) -> Option<String> {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR").replace(
            "/adventofcode",
            &format!("/adventofcode/adventofcode-private/{folder}"),
        ),
        format!("{}.txt", filename),
    ]
    .iter()
    .collect();
    match std::fs::read_to_string(&path) {
        Ok(ok) => Some(ok),
        Err(err) => {
            println!("could not read file {:#?} {:#?}", &path, err);
            None
        }
    }
}

pub fn read_file_line_by_line_to_string(filename: &str) -> Vec<String> {
    read_data_for_day(filename)
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>()
}

pub fn read_file_line_by_line_to_string_test(filename: &str) -> Vec<String> {
    read_test_data_for_day(filename)
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>()
}

pub fn read_file_line_by_line_to_i64(filename: &str) -> Vec<i64> {
    read_data_for_day(filename)
        .unwrap()
        .split("\n")
        .map(|str| str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn read_file_line_by_line_to_i64_test(filename: &str) -> Vec<i64> {
    read_test_data_for_day(filename)
        .unwrap()
        .split("\n")
        .map(|str| str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::flip_lines_vec_to_column_vec;
/// let input = vec![
///     String::from("00100"),
///     String::from("11110"),
///     String::from("10110"),
/// ];
/// let expected_output = vec![
///     String::from("011"),
///     String::from("010"),
///     String::from("111"),
///     String::from("011"),
///     String::from("000"),
/// ];
/// assert_eq!(expected_output, flip_lines_vec_to_column_vec(&input));
/// ```
pub fn flip_lines_vec_to_column_vec(lines: &Vec<String>) -> Vec<String> {
    let mut columns: Vec<String> = vec![String::from(""); lines[0].len()];
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let ch = lines[i].chars().nth(j).unwrap();
            columns[j].push(ch);
        }
    }
    columns
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::flip_vec_of_vec_of_numbers;
/// let input = vec![
///     vec![22, 13, 17, 11, 0],
///     vec![8, 2, 23, 4, 24],
///     vec![21, 9, 14, 16, 7],
///     vec![6, 10, 3, 18, 5],
///     vec![1, 12, 20, 15, 19],
/// ];
/// let expected_output = vec![
///     vec![22, 8, 21, 6, 1],
///     vec![13, 2, 9, 10, 12],
///     vec![17, 23, 14, 3, 20],
///     vec![11, 4, 16, 18, 15],
///     vec![0, 24, 7, 5, 19],
/// ];
/// assert_eq!(expected_output, flip_vec_of_vec_of_numbers(input));
///
/// let input_two = vec![
///     vec![0,0,1,0,0],
///     vec![1,1,1,1,0],
///     vec![1,0,1,1,0]
/// ];
/// let expected_output_two = vec![
///     vec![0,1,1],
///     vec![0,1,0],
///     vec![1,1,1],
///     vec![0,1,1],
///     vec![0,0,0]
/// ];
/// assert_eq!(expected_output_two, flip_vec_of_vec_of_numbers(input_two));
/// ```
pub fn flip_vec_of_vec_of_numbers(grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_rows = vec![vec![0; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            new_rows[j][i] = grid[i][j];
        }
    }
    new_rows
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::amount_of_chars_in_string_and_sum_all_chars_to_u32;
/// let input = String::from("0010010111");
/// let expected_output = (10,5);
/// assert_eq!(expected_output, amount_of_chars_in_string_and_sum_all_chars_to_u32(&input));
/// ```
pub fn amount_of_chars_in_string_and_sum_all_chars_to_u32(data: &String) -> (u32, u32) {
    let chars = data.chars().collect::<Vec<char>>();
    let sum = chars.iter().map(|char| char.to_digit(10).unwrap()).sum();
    (chars.len() as u32, sum)
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::split_vec_of_string_by_empty_line;
/// let input = vec![
///        String::from("Text1"),
///        String::from("Text2"),
///        String::from(""),
///        String::from("Text3"),
///        String::from("Text4"),
/// ];
/// let expected_output = vec![
///     vec![String::from("Text1"), String::from("Text2")],
///     vec![String::from("Text3"), String::from("Text4")],
/// ];
/// assert_eq!(expected_output, split_vec_of_string_by_empty_line(input));
///
/// let input_two = vec![
///        String::from("Text1"),
///        String::from("Text2"),
///        String::from(""),
///        String::from("Text3"),
///        String::from("Text4"),
///        String::from(""),
/// ];
/// let expected_output_two = vec![
///     vec![String::from("Text1"), String::from("Text2")],
///     vec![String::from("Text3"), String::from("Text4")],
/// ];
/// assert_eq!(expected_output_two, split_vec_of_string_by_empty_line(input_two));
/// ```
pub fn split_vec_of_string_by_empty_line(data: Vec<String>) -> Vec<Vec<String>> {
    let mut output = Vec::<Vec<String>>::new();
    let mut section = Vec::<String>::new();
    for i in 0..data.len() {
        let line = data[i].clone();
        if line.is_empty() && i == (data.len() - 1) {
            output.push(section.clone());
            return output;
        }
        if line.is_empty() {
            output.push(section.clone());
            section.clear();
        } else {
            section.push(line);
        }
    }
    output.push(section.clone());
    section.clear();
    output
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::split_slice_of_string_by_empty_line;
/// let input = &[
///        String::from("Text1"),
///        String::from("Text2"),
///        String::from(""),
///        String::from("Text3"),
///        String::from("Text4"),
/// ];
/// let expected_output = vec![
///     vec![String::from("Text1"), String::from("Text2")],
///     vec![String::from("Text3"), String::from("Text4")],
/// ];
/// assert_eq!(expected_output, split_slice_of_string_by_empty_line(input));
///
/// let input_two = &[
///        String::from("Text1"),
///        String::from("Text2"),
///        String::from(""),
///        String::from("Text3"),
///        String::from("Text4"),
///        String::from(""),
/// ];
/// let expected_output_two = vec![
///     vec![String::from("Text1"), String::from("Text2")],
///     vec![String::from("Text3"), String::from("Text4")],
/// ];
/// assert_eq!(expected_output_two, split_slice_of_string_by_empty_line(input_two));
/// ```
pub fn split_slice_of_string_by_empty_line(data: &[String]) -> Vec<Vec<String>> {
    split_vec_of_string_by_empty_line(data.to_vec())
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::replace_space_in_strings_with_comma;
/// let input = &vec![
///        String::from("22 13 17 11  0"),
///        String::from("8  2 23  4 24")
/// ];
/// let expected_output = vec![
///        String::from("22,13,17,11,0"),
///        String::from("8,2,23,4,24")
/// ];;
/// assert_eq!(expected_output, replace_space_in_strings_with_comma(input));
/// ```
pub fn replace_space_in_strings_with_comma(data: &Vec<String>) -> Vec<String> {
    data.iter()
        .map(|line| replace_space_in_string_with_comma(line))
        .collect()
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::replace_space_in_string_with_comma;
/// let input = String::from("22 13 17 11  0");
/// let expected_output = String::from("22,13,17,11,0");
/// assert_eq!(expected_output, replace_space_in_string_with_comma(&input));
/// ```
pub fn replace_space_in_string_with_comma(data: &String) -> String {
    data.split_whitespace().collect::<Vec<_>>().join(",")
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::comma_separated_strings_of_numbers_to_u32_vec;
/// let input = &vec![
///         String::from("7,4,9,5,11,17,23,2,0,14"),
///         String::from("7,4,9,5,11,17,23,2,0,14")
/// ];
/// let expected_output = vec![
///         vec![7,4,9,5,11,17,23,2,0,14],
///         vec![7,4,9,5,11,17,23,2,0,14]
/// ];
/// assert_eq!(expected_output, comma_separated_strings_of_numbers_to_u32_vec(input));
/// ```
pub fn comma_separated_strings_of_numbers_to_u32_vec(data: &Vec<String>) -> Vec<Vec<u32>> {
    data.iter()
        .map(|line| comma_separated_string_of_numbers_to_u32_vec(line))
        .collect::<Vec<Vec<u32>>>()
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::comma_separated_strings_of_numbers_to_u8_vec;
/// let input = &vec![
///         String::from("7,4,9,5,11,17,23,2,0,14"),
///         String::from("7,4,9,5,11,17,23,2,0,14")
/// ];
/// let expected_output = vec![
///         vec![7,4,9,5,11,17,23,2,0,14],
///         vec![7,4,9,5,11,17,23,2,0,14]
/// ];
/// assert_eq!(expected_output, comma_separated_strings_of_numbers_to_u8_vec(input));
/// ```
pub fn comma_separated_strings_of_numbers_to_u8_vec(data: &Vec<String>) -> Vec<Vec<u8>> {
    data.iter()
        .map(|line| comma_separated_string_of_numbers_to_u8_vec(line))
        .collect::<Vec<Vec<u8>>>()
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::comma_separated_string_of_numbers_to_u32_vec;
/// let input = String::from("7,4,9,5,11,17,23,2,0,14");
/// let expected_output = vec![7,4,9,5,11,17,23,2,0,14];
/// assert_eq!(expected_output, comma_separated_string_of_numbers_to_u32_vec(&input));
/// ```
pub fn comma_separated_string_of_numbers_to_u32_vec(data: &String) -> Vec<u32> {
    data.split(",")
        .map(|char| char.parse::<u32>().unwrap())
        .collect()
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::comma_separated_string_of_numbers_to_u8_vec;
/// let input = String::from("7,4,9,5,11,17,23,2,0,14");
/// let expected_output = vec![7,4,9,5,11,17,23,2,0,14];
/// assert_eq!(expected_output, comma_separated_string_of_numbers_to_u8_vec(&input));
/// ```
pub fn comma_separated_string_of_numbers_to_u8_vec(data: &String) -> Vec<u8> {
    data.split(",")
        .map(|char| char.parse::<u8>().unwrap())
        .collect()
}

/// # Examples
///
/// Basic usage:
///
/// ```
/// use aoc2021::util::string_of_between_zero_and_nine_to_to_usize_vec;
/// let input = vec![
///         String::from("2199943210"),
///         String::from("3987894921"),
/// ];
/// let expected_output = vec![
///         vec![2,1,9,9,9,4,3,2,1,0],
///         vec![3,9,8,7,8,9,4,9,2,1],
/// ];
/// assert_eq!(expected_output, string_of_between_zero_and_nine_to_to_usize_vec(input));
/// ```
pub fn string_of_between_zero_and_nine_to_to_usize_vec(data: Vec<String>) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    data.iter()
        .map(|row| {
            row.chars()
                .map(|char| char.to_digit(RADIX).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
