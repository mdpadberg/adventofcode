use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("10").unwrap()));
}

fn solve(input: String) -> i32 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::from_str(line))
        .collect();
    let mut cpu_cycles = vec![Cpu {
        cycle: 0,
        x_register: 1,
    }];
    for instruction in instructions {
        let last = cpu_cycles.last().unwrap();
        match instruction {
            Instruction::Noop => {
                cpu_cycles.push(Cpu {
                    cycle: last.cycle + 1,
                    x_register: last.x_register,
                });
            }
            Instruction::Addx(x) => {
                cpu_cycles.push(Cpu {
                    cycle: last.cycle + 2,
                    x_register: last.x_register + x,
                });
            }
        }
    }
    (20..=220)
        .step_by(40)
        .map(|i: i32| {
            cpu_cycles
                .iter()
                .rev()
                .find(
                    |Cpu {
                         cycle,
                         x_register: _,
                     }| *cycle < i,
                )
                .unwrap()
                .x_register
                * i
        })
        .sum()
}

#[derive(Debug)]
struct Cpu {
    cycle: i32,
    x_register: i32,
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
        assert_eq!(13140, solve(read_test_data_for_day("10-1").unwrap()));
    }
}
