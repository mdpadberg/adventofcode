use aoc2021::util::{
    read_file_line_by_line_to_string, string_of_between_zero_and_nine_to_to_usize_vec,
};
use std::collections::{HashMap, HashSet};

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("11"))
    );
}

fn solve_part_two(input: Vec<String>) -> usize {
    let mut grid = Grid::create(input);
    let mut sum = 0;
    let mut rounds = 0;
    while sum != 100 {
        sum = grid.round();
        rounds += 1;
    }
    rounds
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Cordinate {
    y: isize,
    x: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Octopus {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    octopuses: HashMap<Cordinate, Octopus>,
}

impl Grid {
    fn create(input: Vec<String>) -> Grid {
        let data = string_of_between_zero_and_nine_to_to_usize_vec(input);
        let mut octopuses = HashMap::new();
        for y in 0..data.len() {
            for x in 0..data[y].len() {
                octopuses.insert(
                    Cordinate {
                        y: y as isize,
                        x: x as isize,
                    },
                    Octopus {
                        score: data[y][x] as u32,
                    },
                );
            }
        }
        Grid { octopuses }
    }

    fn round(&mut self) -> usize {
        self.step_one();
        let amount_of_flashing_octopuses = self.step_two();
        self.step_three();
        amount_of_flashing_octopuses
    }

    fn step_one(&mut self) {
        for (_, octopus) in &mut self.octopuses {
            octopus.score += 1;
        }
    }

    fn step_two(&mut self) -> usize {
        let mut octopuses_that_flash = HashSet::<Cordinate>::new();
        loop {
            let flashing_octopuses: HashMap<Cordinate, Octopus> = self
                .octopuses
                .clone()
                .into_iter()
                .filter(|(cordinate, octopus)| {
                    octopus.score > 9 && !octopuses_that_flash.contains(cordinate)
                })
                .collect();
            if flashing_octopuses.is_empty() {
                break;
            }
            for (cordinates, octopus) in &flashing_octopuses {
                let adjacent_cordinates = [
                    Cordinate {
                        y: cordinates.y - 1,
                        x: cordinates.x - 1,
                    },
                    Cordinate {
                        y: cordinates.y - 1,
                        x: cordinates.x,
                    },
                    Cordinate {
                        y: cordinates.y - 1,
                        x: cordinates.x + 1,
                    },
                    Cordinate {
                        y: cordinates.y,
                        x: cordinates.x - 1,
                    },
                    Cordinate {
                        y: cordinates.y,
                        x: cordinates.x + 1,
                    },
                    Cordinate {
                        y: cordinates.y + 1,
                        x: cordinates.x - 1,
                    },
                    Cordinate {
                        y: cordinates.y + 1,
                        x: cordinates.x,
                    },
                    Cordinate {
                        y: cordinates.y + 1,
                        x: cordinates.x + 1,
                    },
                ];
                for adjacent_cordinate in adjacent_cordinates {
                    self.octopuses
                        .entry(adjacent_cordinate)
                        .and_modify(|octopus| octopus.score += 1);
                }
            }
            octopuses_that_flash.extend(flashing_octopuses.keys());
        }
        octopuses_that_flash.len()
    }

    fn step_three(&mut self) {
        for (_, octopus) in &mut self.octopuses {
            if octopus.score > 9 {
                octopus.score = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn two() {
        assert_eq!(195, solve_part_two(read_file_line_by_line_to_string_test("11-0")));
    }
}
