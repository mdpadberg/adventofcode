use aoc2021::util::read_file_line_by_line_to_i64;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_i64("2021/data/1.txt"))
    );
}

fn solve_part_one(data: Vec<i64>) -> u64 {
    let mut increased = 0;
    for i in 1..data.len() {
        if data[i] - data[i - 1] > 0 {
            increased += 1;
        }
    }
    increased
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, solve_part_one(data));
    }
}
