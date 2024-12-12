use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    println!("{:?}", solve(read_data_for_day("10").unwrap()));
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coordinates {
    x: i8,
    y: i8,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Trail(Vec<Coordinates>);

impl Trail {
    fn get_current_coordinates_and_current_number(
        &self,
        grid: &HashMap<Coordinates, u32>,
    ) -> Option<(Coordinates, u32)> {
        let current_coordinates = self.0.last()?;
        let current_number = grid.get(current_coordinates)?;
        Some((*current_coordinates, *current_number))
    }
}

fn solve(input: String) -> usize {
    let mut starting_points: HashMap<Coordinates, u32> = HashMap::new();
    let grid = input
        .split("\n")
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, value)| {
                    value.to_digit(10).and_then(|value| {
                        let coordinates = Coordinates {
                            x: x as i8,
                            y: y as i8,
                        };
                        if value == 0 {
                            starting_points.insert(coordinates, value);
                        }
                        Some((coordinates, value))
                    })
                })
                .collect::<HashMap<Coordinates, u32>>()
        })
        .collect::<HashMap<Coordinates, u32>>();
    let mut points: BTreeSet<Trail> = starting_points
        .into_iter()
        .map(|(coordinates, _)| Trail(vec![coordinates]))
        .collect();
    let mut valid_trails: HashSet<Trail> = HashSet::new();
    while let Some(trail) = points.pop_first() {
        if let Some((current_coordinates, current_number)) =
            trail.get_current_coordinates_and_current_number(&grid)
        {
            vec![
                //left
                Coordinates {
                    x: current_coordinates.x - 1,
                    y: current_coordinates.y,
                },
                //up
                Coordinates {
                    x: current_coordinates.x,
                    y: current_coordinates.y + 1,
                },
                //right
                Coordinates {
                    x: current_coordinates.x + 1,
                    y: current_coordinates.y,
                },
                //down
                Coordinates {
                    x: current_coordinates.x,
                    y: current_coordinates.y - 1,
                },
            ]
            .into_iter()
            .filter_map(|new_coordinates| {
                grid.get(&new_coordinates)
                    .and_then(|new_value| Some((new_coordinates, new_value)))
            })
            .for_each(|(new_coordinates, new_number)| {
                let mut new_trail = trail.0.clone();
                new_trail.push(new_coordinates);
                if current_number == 8 && new_number == &9 {
                    valid_trails.insert(Trail(new_trail));
                } else if &(current_number + 1) == new_number {
                    points.insert(Trail(new_trail));
                }
            });
        }
    }
    valid_trails.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(3, solve(read_test_data_for_day("10-5").unwrap()));
        assert_eq!(13, solve(read_test_data_for_day("10-7").unwrap()));
        assert_eq!(227, solve(read_test_data_for_day("10-8").unwrap()));
        assert_eq!(81, solve(read_test_data_for_day("10-9").unwrap()));
    }
}
