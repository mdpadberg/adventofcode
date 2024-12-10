use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use std::{cmp::max, collections::HashSet};

fn main() {
    println!("{}", solve(read_data_for_day("17").unwrap()));
}

//TODO speed up and clean up code before starting on 17b
fn solve(input: String) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let mut rocks: HashSet<Rock> = HashSet::new();
    let mut current_rock: Rock = Rock::get_rock(TypeOfRock::ONE, &DropZone::new(0));
    let mut i = 0;
    while rocks.len() < 2022 {
        let jet = Jet::new(chars[i % chars.len()]);
        i += 1;
        if !current_rock.at_edge_of_playing_area(&jet) {
            if current_rock.can_be_pushed_by_jet(&jet, &rocks) {
                current_rock.push_by_jet(&jet);
            }
        }
        if current_rock.can_fall(&rocks) {
            current_rock.fall();
        } else {
            rocks.insert(current_rock.clone());
            current_rock = current_rock.next_rock(&rocks);
        }
        // print(&rocks, &current_rock);
    }
    rocks
        .into_iter()
        .flat_map(|r| r.coordinates)
        .map(|c| c.y)
        .max()
        .unwrap()
}

// fn print(rocks: &HashSet<Rock>, current_rock: &Rock) {
//     let height: u32 = max(
//         rocks
//             .into_iter()
//             .flat_map(|r| r.coordinates.clone())
//             .map(|c| c.y)
//             .max()
//             .unwrap_or(0),
//         current_rock
//             .coordinates
//             .iter()
//             .map(|c| c.y)
//             .max()
//             .unwrap_or(0),
//     ) + 1;
//     let mut grid = vec![vec!["."; 7]; height as usize];
//     let len = grid.len() - 1;

//     for rock in rocks {
//         for coordinate in &rock.coordinates {
//             grid[(coordinate.y) as usize][coordinate.x as usize] = "#"
//         }
//     }

//     for coordinate in &current_rock.coordinates {
//         grid[(coordinate.y) as usize][coordinate.x as usize] = "@"
//     }

//     for (i, row) in grid.into_iter().rev().enumerate() {
//         if i == len {
//             println!("---------");
//         } else {
//             println!("{:?}", row.into_iter().join(""));
//         }
//     }
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Rock {
    coordinates: Vec<Coordinate>,
    type_of_rock: TypeOfRock,
}

impl Rock {
    fn at_edge_of_playing_area(&self, jet: &Jet) -> bool {
        match jet {
            Jet::LEFT => !self.coordinates.iter().all(|c| c.x > 0),
            Jet::RIGHT => !self.coordinates.iter().all(|c| c.x < 6),
        }
    }

    fn can_be_pushed_by_jet(&self, jet: &Jet, rocks: &HashSet<Rock>) -> bool {
        let coordinates: HashSet<Coordinate> = rocks
            .iter()
            .flat_map(|rock| rock.coordinates.clone())
            .collect::<HashSet<Coordinate>>();
        let x_direction: i32 = match jet {
            Jet::LEFT => -1,
            Jet::RIGHT => 1,
        };
        self.coordinates.iter().all(|c| {
            !coordinates.contains(&Coordinate {
                y: c.y,
                x: ((c.x as i32) + x_direction) as u32,
            })
        })
    }

    fn push_by_jet(&mut self, jet: &Jet) {
        for coordinate in self.coordinates.iter_mut() {
            match jet {
                Jet::LEFT => {
                    coordinate.x -= 1;
                }
                Jet::RIGHT => {
                    coordinate.x += 1;
                }
            }
        }
    }

    fn can_fall(&self, rocks: &HashSet<Rock>) -> bool {
        let coordinates: HashSet<Coordinate> = rocks
            .iter()
            .flat_map(|rock| rock.coordinates.clone())
            .collect::<HashSet<Coordinate>>();
        self.coordinates
            .iter()
            .all(|c| c.y > 1 && !coordinates.contains(&Coordinate { y: c.y - 1, x: c.x }))
    }

    fn fall(&mut self) {
        for coordinate in self.coordinates.iter_mut() {
            coordinate.y -= 1;
        }
    }

    fn next_rock(&self, rocks: &HashSet<Rock>) -> Rock {
        let dropzone = DropZone::new(
            rocks
                .iter()
                .flat_map(|r| r.coordinates.iter().map(|c| c.y))
                .max()
                .unwrap(),
        );
        match self.type_of_rock {
            TypeOfRock::ONE => Rock::get_rock(TypeOfRock::TWO, &dropzone),
            TypeOfRock::TWO => Rock::get_rock(TypeOfRock::THREE, &dropzone),
            TypeOfRock::THREE => Rock::get_rock(TypeOfRock::FOUR, &dropzone),
            TypeOfRock::FOUR => Rock::get_rock(TypeOfRock::FIVE, &dropzone),
            TypeOfRock::FIVE => Rock::get_rock(TypeOfRock::ONE, &dropzone),
        }
    }

    fn get_rock(type_of_rock: TypeOfRock, dropzone: &DropZone) -> Rock {
        match type_of_rock {
            //|..@@@@.|
            //|.......|
            //|.......|
            //|.......|
            //|#######|
            TypeOfRock::ONE => Rock {
                coordinates: vec![
                    Coordinate {
                        y: dropzone.y,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y,
                        x: 3,
                    },
                    Coordinate {
                        y: dropzone.y,
                        x: 4,
                    },
                    Coordinate {
                        y: dropzone.y,
                        x: 5,
                    },
                ],
                type_of_rock,
            },
            //|...@...|
            //|..@@@..|
            //|...@...|
            //|.......|
            //|.......|
            //|.......|
            //|#######|
            TypeOfRock::TWO => Rock {
                coordinates: vec![
                    Coordinate {
                        y: dropzone.y,
                        x: 3,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 3,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 4,
                    },
                    Coordinate {
                        y: dropzone.y + 2,
                        x: 3,
                    },
                ],
                type_of_rock,
            },
            //|....@..|
            //|....@..|
            //|..@@@..|
            //|.......|
            //|.......|
            //|.......|
            //|#######|
            TypeOfRock::THREE => Rock {
                coordinates: vec![
                    Coordinate {
                        y: dropzone.y,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y,
                        x: 3,
                    },
                    Coordinate {
                        y: dropzone.y,
                        x: 4,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 4,
                    },
                    Coordinate {
                        y: dropzone.y + 2,
                        x: 4,
                    },
                ],
                type_of_rock,
            },
            //|..@....|
            //|..@....|
            //|..@....|
            //|..@....|
            //|.......|
            //|.......|
            //|.......|
            //|#######|
            TypeOfRock::FOUR => Rock {
                coordinates: vec![
                    Coordinate {
                        y: dropzone.y,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y + 2,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y + 3,
                        x: 2,
                    },
                ],
                type_of_rock,
            },
            //|..@@...|
            //|..@@...|
            //|.......|
            //|.......|
            //|.......|
            //|#######|
            TypeOfRock::FIVE => Rock {
                coordinates: vec![
                    Coordinate {
                        y: dropzone.y,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y,
                        x: 3,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 2,
                    },
                    Coordinate {
                        y: dropzone.y + 1,
                        x: 3,
                    },
                ],
                type_of_rock,
            },
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    y: u32,
    x: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum TypeOfRock {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

#[derive(Debug)]
struct DropZone {
    y: u32,
}

impl DropZone {
    fn new(last_heighest_y: u32) -> DropZone {
        DropZone {
            y: last_heighest_y + 4,
        }
    }
}

#[derive(Debug)]
enum Jet {
    LEFT,
    RIGHT,
}

impl Jet {
    fn new(c: char) -> Jet {
        match c {
            '<' => Jet::LEFT,
            '>' => Jet::RIGHT,
            _ => panic!("unsupported jet operation {}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(3068, solve(read_test_data_for_day("17-1").unwrap()));
    }
}
