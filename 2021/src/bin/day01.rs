use aoc2021::util::read_file_line_by_line_to_i64;
use measure_time::print_time;

fn main() {
    print_time!("execution");
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_i64("2021/data/1.txt"))
    );
    println!(
        "solve_part_two -> {:#?}",
        solve_part_two(read_file_line_by_line_to_i64("2021/data/1.txt"))
    );
    //cargo run --bin day01 --release
    //execution took 0.06ms

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
    fn one() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, solve_part_one(data));
    }

    #[test]
    fn two() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, solve_part_two(data));
    }
}
