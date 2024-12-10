use aoc2021::util::read_file_line_by_line_to_i64;

fn main() {
    println!(
        "solve_part_two -> {:#?}",
        solve_part_two(read_file_line_by_line_to_i64("1"))
    );
}

fn solve_part_two(data: Vec<i64>) -> u64 {
    let mut increased = 0;
    for i in 0..data.len()-3 {
        if data[i] + data[i+1] + data[i+2] < data[i+1] + data[i+2] + data[i+3] {
            increased += 1;
        }
    }
    increased
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, solve_part_two(data));
    }
}
