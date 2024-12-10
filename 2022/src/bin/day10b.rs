use aoc2022::util::read_data_for_day;
use itertools::Itertools;

///"###...##..####.####.#..#.#..#.###..#..#.",
///"#..#.#..#....#.#....#..#.#..#.#..#.#.#..",
///"#..#.#......#..###..####.#..#.#..#.##...",
///"###..#.##..#...#....#..#.#..#.###..#.#..",
///"#.#..#..#.#....#....#..#.#..#.#.#..#.#..",
///"#..#..###.####.####.#..#..##..#..#.#..#.",
///".",
fn main() {
    println!("{:#?}", solve(read_data_for_day("10").unwrap()));
}

fn solve(input: String) -> Vec<String> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::from_str(line))
        .collect();
    let mut cpu_cycles = vec![Cpu {
        cycle: 0,
        sprite: 1,
    }];
    for instruction in instructions {
        let last_cycle = cpu_cycles.last().unwrap().cycle;
        let last_x_register = cpu_cycles.last().unwrap().sprite;
        match instruction {
            Instruction::Noop => {
                cpu_cycles.push(Cpu {
                    cycle: last_cycle + 1,
                    sprite: last_x_register,
                });
            }
            Instruction::Addx(x) => {
                cpu_cycles.push(Cpu {
                    cycle: last_cycle + 1,
                    sprite: last_x_register,
                });
                cpu_cycles.push(Cpu {
                    cycle: last_cycle + 2,
                    sprite: last_x_register + x,
                });
            }
        }
    }
    cpu_cycles
        .iter()
        .map(|cpu| cpu.to_pixel())
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}

#[derive(Debug)]
struct Cpu {
    cycle: i32,
    sprite: i32,
}

impl Cpu {
    fn to_pixel(&self) -> char {
        if (self.cycle % 40 <= (self.sprite + 1)) && ((self.sprite - 1) <= self.cycle % 40) {
            '#'
        } else {
            '.'
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_str(str: &str) -> Instruction {
        if str == "noop" {
            Instruction::Noop
        } else {
            Instruction::Addx(str.get(5..).unwrap().parse::<i32>().unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        let result = vec![
            String::from("##..##..##..##..##..##..##..##..##..##.."),
            String::from("###...###...###...###...###...###...###."),
            String::from("####....####....####....####....####...."),
            String::from("#####.....#####.....#####.....#####....."),
            String::from("######......######......######......####"),
            String::from("#######.......#######.......#######....."),
            String::from("."),
        ];
        assert_eq!(result, solve(read_test_data_for_day("10-1").unwrap()));
    }
}
