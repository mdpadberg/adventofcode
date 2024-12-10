use aoc2021::util::{
    read_file_line_by_line_to_string, string_of_between_zero_and_nine_to_to_usize_vec,
};
use itertools::Itertools;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("9"))
    );
}

//TEST WORKS, REAL DATA -> thread 'main' has overflowed its stack, fatal runtime error: stack overflow
fn solve_part_two(rows: Vec<String>) -> usize {
    let grid = Grid::create(string_of_between_zero_and_nine_to_to_usize_vec(rows));
    let basins = grid.get_all_values_below_row_avg()
        .into_iter()
        .filter(|point| !grid.has_lower_neighbors(point))
        .map(|lowest_point| grid.floodfill(&lowest_point))
        .map(|basin| basin.iter().map(|point| point.value).count())
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<usize>>();
    basins[0] * basins[1] * basins[2]
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Grid {
    data: Vec<Vec<Point>>,
    avg_per_row: Vec<u32>,
}

impl Grid {
    fn create(input: Vec<Vec<u32>>) -> Grid {
        let mut values = Vec::<Vec<Point>>::new();
        let mut avg_per_row = Vec::<u32>::new();
        for i in 0..input.len() {
            let mut row = Vec::<Point>::new();
            let mut row_sum = 0;
            for j in 0..input[i].len() {
                let value = input[i][j];
                row_sum += value;
                row.push(Point {
                    x: j as u32,
                    y: i as u32,
                    value,
                })
            }
            avg_per_row.push(row_sum / row.len() as u32);
            values.push(row);
        }
        Grid {
            data: values,
            avg_per_row,
        }
    }

    fn get_all_values_below_row_avg(&self) -> Vec<Point> {
        self.data
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|point| point.value <= self.avg_per_row[point.y as usize] + 1)
                    .map(|value| value.to_owned())
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    fn has_lower_neighbors(&self, current_point: &Point) -> bool {
        let x = current_point.x as i32;
        let y = current_point.y as i32;
        let left = self.get_point(x - 1, y);
        let right = self.get_point(x + 1, y);
        let below = self.get_point(x, y + 1);
        let up = self.get_point(x, y - 1);
        [left, right, below, up]
            .iter()
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .filter(|point| point.value <= current_point.value)
            .collect::<Vec<Point>>()
            .len()
            > 0
    }

    fn floodfill(&self, current_point: &Point) -> Vec<Point> {
        let mut recursive_neighbors = self.recursive_neighbors(current_point);
        recursive_neighbors.push(current_point.clone());
        recursive_neighbors
    }

    fn recursive_neighbors(&self, current_point: &Point) -> Vec<Point> {
        let x = current_point.x as i32;
        let y = current_point.y as i32;
        let left = self.get_point(x - 1, y);
        let right = self.get_point(x + 1, y);
        let below = self.get_point(x, y + 1);
        let up = self.get_point(x, y - 1);
        let neighbors_which_are_not_nine_and_higher_then_current_value = [left, right, below, up]
            .iter()
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .filter(|point| point.value != 9)
            .filter(|point| point.value >= current_point.value)
            .collect::<Vec<Point>>();
        if neighbors_which_are_not_nine_and_higher_then_current_value.len() == 0 {
            return vec![];
        }
        let recursive_neighbors = neighbors_which_are_not_nine_and_higher_then_current_value
            .iter()
            .flat_map(|point| self.floodfill(point))
            .collect::<Vec<Point>>();
        neighbors_which_are_not_nine_and_higher_then_current_value
            .into_iter()
            .chain(recursive_neighbors)
            .sorted()
            .dedup()
            .collect()
    }

    fn get_point(&self, x: i32, y: i32) -> Option<Point> {
        if y >= 0 && (y as usize) < self.data.len() {
            let row = &self.data[y as usize];
            if x >= 0 && (x as usize) < row.len() {
                return Some(row[x as usize]);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
    value: u32,
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn two() {
        assert_eq!(1134, solve_part_two(read_file_line_by_line_to_string_test("9-0")));
    }

    #[test]
    fn create_grid_test() {
        let expected_output = get_test_grid_object();
        assert_eq!(
            expected_output,
            Grid::create(string_of_between_zero_and_nine_to_to_usize_vec(read_file_line_by_line_to_string_test("9-0")))
        );
    }

    #[test]
    fn get_all_values_below_row_avg_test() {
        let expected_output = vec![
            Point {
                x: 0,
                y: 0,
                value: 2,
            },
            Point {
                x: 1,
                y: 0,
                value: 1,
            },
            Point {
                x: 5,
                y: 0,
                value: 4,
            },
            Point {
                x: 6,
                y: 0,
                value: 3,
            },
            Point {
                x: 7,
                y: 0,
                value: 2,
            },
            Point {
                x: 8,
                y: 0,
                value: 1,
            },
            Point {
                x: 9,
                y: 0,
                value: 0,
            },
            Point {
                x: 0,
                y: 1,
                value: 3,
            },
            Point {
                x: 3,
                y: 1,
                value: 7,
            },
            Point {
                x: 6,
                y: 1,
                value: 4,
            },
            Point {
                x: 8,
                y: 1,
                value: 2,
            },
            Point {
                x: 9,
                y: 1,
                value: 1,
            },
            Point {
                x: 1,
                y: 2,
                value: 8,
            },
            Point {
                x: 2,
                y: 2,
                value: 5,
            },
            Point {
                x: 3,
                y: 2,
                value: 6,
            },
            Point {
                x: 4,
                y: 2,
                value: 7,
            },
            Point {
                x: 5,
                y: 2,
                value: 8,
            },
            Point {
                x: 7,
                y: 2,
                value: 8,
            },
            Point {
                x: 9,
                y: 2,
                value: 2,
            },
            Point {
                x: 0,
                y: 3,
                value: 8,
            },
            Point {
                x: 1,
                y: 3,
                value: 7,
            },
            Point {
                x: 2,
                y: 3,
                value: 6,
            },
            Point {
                x: 3,
                y: 3,
                value: 7,
            },
            Point {
                x: 4,
                y: 3,
                value: 8,
            },
            Point {
                x: 6,
                y: 3,
                value: 6,
            },
            Point {
                x: 7,
                y: 3,
                value: 7,
            },
            Point {
                x: 8,
                y: 3,
                value: 8,
            },
            Point {
                x: 1,
                y: 4,
                value: 8,
            },
            Point {
                x: 5,
                y: 4,
                value: 6,
            },
            Point {
                x: 6,
                y: 4,
                value: 5,
            },
            Point {
                x: 7,
                y: 4,
                value: 6,
            },
            Point {
                x: 8,
                y: 4,
                value: 7,
            },
            Point {
                x: 9,
                y: 4,
                value: 8,
            },
        ];
        assert_eq!(
            expected_output,
            get_test_grid_object().get_all_values_below_row_avg()
        )
    }

    #[test]
    fn get_point_test() {
        assert_eq!(
            Some(Point {
                x: 0,
                y: 0,
                value: 2
            }),
            get_test_grid_object().get_point(0, 0)
        );
        assert_eq!(
            Some(Point {
                x: 4,
                y: 0,
                value: 9
            }),
            get_test_grid_object().get_point(4, 0)
        );
        assert_eq!(
            Some(Point {
                x: 9,
                y: 4,
                value: 8
            }),
            get_test_grid_object().get_point(9, 4)
        );
        assert_eq!(None, get_test_grid_object().get_point(10, 4));
        assert_eq!(None, get_test_grid_object().get_point(9, 5));
    }

    #[test]
    fn get_get_lowest_neighbor_test() {
        assert_eq!(
            true,
            get_test_grid_object().has_lower_neighbors(&Point {
                x: 0,
                y: 0,
                value: 2
            })
        );
        assert_eq!(
            false,
            get_test_grid_object().has_lower_neighbors(&Point {
                x: 1,
                y: 0,
                value: 1
            })
        );
        assert_eq!(
            true,
            get_test_grid_object().has_lower_neighbors(&Point {
                x: 0,
                y: 0,
                value: 2
            })
        );
        assert_eq!(
            false,
            get_test_grid_object().has_lower_neighbors(&Point {
                x: 9,
                y: 0,
                value: 0
            })
        );
        assert_eq!(
            false,
            get_test_grid_object().has_lower_neighbors(&Point {
                x: 6,
                y: 4,
                value: 5
            })
        );
        assert_eq!(
            true,
            get_test_grid_object().has_lower_neighbors(&Point {
                x: 9,
                y: 4,
                value: 8
            })
        );
        assert_eq!(
            true,
            get_test_grid_object_two().has_lower_neighbors(&Point {
                x: 1,
                y: 1,
                value: 9
            })
        );
    }

    #[test]
    fn floodfill_test() {
        assert_eq!(
            vec![
                Point {
                    x: 0,
                    y: 0,
                    value: 2
                },
                Point {
                    x: 0,
                    y: 1,
                    value: 3
                },
                Point {
                    x: 1,
                    y: 0,
                    value: 1
                }
            ],
            Grid::floodfill(
                &get_test_grid_object(),
                &Point {
                    x: 1,
                    y: 0,
                    value: 1
                }
            )
        );
        assert_eq!(
            vec![
                Point {
                    x: 5,
                    y: 0,
                    value: 4
                },
                Point {
                    x: 6,
                    y: 0,
                    value: 3
                },
                Point {
                    x: 6,
                    y: 1,
                    value: 4
                },
                Point {
                    x: 7,
                    y: 0,
                    value: 2
                },
                Point {
                    x: 8,
                    y: 0,
                    value: 1
                },
                Point {
                    x: 8,
                    y: 1,
                    value: 2
                },
                Point {
                    x: 9,
                    y: 1,
                    value: 1
                },
                Point {
                    x: 9,
                    y: 2,
                    value: 2
                },
                Point {
                    x: 9,
                    y: 0,
                    value: 0
                }
            ],
            Grid::floodfill(
                &get_test_grid_object(),
                &Point {
                    x: 9,
                    y: 0,
                    value: 0
                }
            )
        );
        assert_eq!(
            vec![
                Point {
                    x: 0,
                    y: 3,
                    value: 8
                },
                Point {
                    x: 1,
                    y: 2,
                    value: 8
                },
                Point {
                    x: 1,
                    y: 3,
                    value: 7
                },
                Point {
                    x: 1,
                    y: 4,
                    value: 8
                },
                Point {
                    x: 2,
                    y: 1,
                    value: 8
                },
                Point {
                    x: 2,
                    y: 3,
                    value: 6
                },
                Point {
                    x: 3,
                    y: 1,
                    value: 7
                },
                Point {
                    x: 3,
                    y: 2,
                    value: 6
                },
                Point {
                    x: 3,
                    y: 3,
                    value: 7
                },
                Point {
                    x: 4,
                    y: 1,
                    value: 8
                },
                Point {
                    x: 4,
                    y: 2,
                    value: 7
                },
                Point {
                    x: 4,
                    y: 3,
                    value: 8
                },
                Point {
                    x: 5,
                    y: 2,
                    value: 8
                },
                Point {
                    x: 2,
                    y: 2,
                    value: 5
                }
            ],
            Grid::floodfill(
                &get_test_grid_object(),
                &Point {
                    x: 2,
                    y: 2,
                    value: 5
                }
            )
        );
        assert_eq!(
            vec![
                Point {
                    x: 5,
                    y: 4,
                    value: 6
                },
                Point {
                    x: 6,
                    y: 3,
                    value: 6
                },
                Point {
                    x: 7,
                    y: 2,
                    value: 8
                },
                Point {
                    x: 7,
                    y: 3,
                    value: 7
                },
                Point {
                    x: 7,
                    y: 4,
                    value: 6
                },
                Point {
                    x: 8,
                    y: 3,
                    value: 8
                },
                Point {
                    x: 8,
                    y: 4,
                    value: 7
                },
                Point {
                    x: 9,
                    y: 4,
                    value: 8
                },
                Point {
                    x: 6,
                    y: 4,
                    value: 5
                }
            ],
            Grid::floodfill(
                &get_test_grid_object(),
                &Point {
                    x: 6,
                    y: 4,
                    value: 5
                }
            )
        );
    }

    fn get_test_grid_object() -> Grid {
        Grid {
            data: vec![
                vec![
                    Point {
                        x: 0,
                        y: 0,
                        value: 2,
                    },
                    Point {
                        x: 1,
                        y: 0,
                        value: 1,
                    },
                    Point {
                        x: 2,
                        y: 0,
                        value: 9,
                    },
                    Point {
                        x: 3,
                        y: 0,
                        value: 9,
                    },
                    Point {
                        x: 4,
                        y: 0,
                        value: 9,
                    },
                    Point {
                        x: 5,
                        y: 0,
                        value: 4,
                    },
                    Point {
                        x: 6,
                        y: 0,
                        value: 3,
                    },
                    Point {
                        x: 7,
                        y: 0,
                        value: 2,
                    },
                    Point {
                        x: 8,
                        y: 0,
                        value: 1,
                    },
                    Point {
                        x: 9,
                        y: 0,
                        value: 0,
                    },
                ],
                vec![
                    Point {
                        x: 0,
                        y: 1,
                        value: 3,
                    },
                    Point {
                        x: 1,
                        y: 1,
                        value: 9,
                    },
                    Point {
                        x: 2,
                        y: 1,
                        value: 8,
                    },
                    Point {
                        x: 3,
                        y: 1,
                        value: 7,
                    },
                    Point {
                        x: 4,
                        y: 1,
                        value: 8,
                    },
                    Point {
                        x: 5,
                        y: 1,
                        value: 9,
                    },
                    Point {
                        x: 6,
                        y: 1,
                        value: 4,
                    },
                    Point {
                        x: 7,
                        y: 1,
                        value: 9,
                    },
                    Point {
                        x: 8,
                        y: 1,
                        value: 2,
                    },
                    Point {
                        x: 9,
                        y: 1,
                        value: 1,
                    },
                ],
                vec![
                    Point {
                        x: 0,
                        y: 2,
                        value: 9,
                    },
                    Point {
                        x: 1,
                        y: 2,
                        value: 8,
                    },
                    Point {
                        x: 2,
                        y: 2,
                        value: 5,
                    },
                    Point {
                        x: 3,
                        y: 2,
                        value: 6,
                    },
                    Point {
                        x: 4,
                        y: 2,
                        value: 7,
                    },
                    Point {
                        x: 5,
                        y: 2,
                        value: 8,
                    },
                    Point {
                        x: 6,
                        y: 2,
                        value: 9,
                    },
                    Point {
                        x: 7,
                        y: 2,
                        value: 8,
                    },
                    Point {
                        x: 8,
                        y: 2,
                        value: 9,
                    },
                    Point {
                        x: 9,
                        y: 2,
                        value: 2,
                    },
                ],
                vec![
                    Point {
                        x: 0,
                        y: 3,
                        value: 8,
                    },
                    Point {
                        x: 1,
                        y: 3,
                        value: 7,
                    },
                    Point {
                        x: 2,
                        y: 3,
                        value: 6,
                    },
                    Point {
                        x: 3,
                        y: 3,
                        value: 7,
                    },
                    Point {
                        x: 4,
                        y: 3,
                        value: 8,
                    },
                    Point {
                        x: 5,
                        y: 3,
                        value: 9,
                    },
                    Point {
                        x: 6,
                        y: 3,
                        value: 6,
                    },
                    Point {
                        x: 7,
                        y: 3,
                        value: 7,
                    },
                    Point {
                        x: 8,
                        y: 3,
                        value: 8,
                    },
                    Point {
                        x: 9,
                        y: 3,
                        value: 9,
                    },
                ],
                vec![
                    Point {
                        x: 0,
                        y: 4,
                        value: 9,
                    },
                    Point {
                        x: 1,
                        y: 4,
                        value: 8,
                    },
                    Point {
                        x: 2,
                        y: 4,
                        value: 9,
                    },
                    Point {
                        x: 3,
                        y: 4,
                        value: 9,
                    },
                    Point {
                        x: 4,
                        y: 4,
                        value: 9,
                    },
                    Point {
                        x: 5,
                        y: 4,
                        value: 6,
                    },
                    Point {
                        x: 6,
                        y: 4,
                        value: 5,
                    },
                    Point {
                        x: 7,
                        y: 4,
                        value: 6,
                    },
                    Point {
                        x: 8,
                        y: 4,
                        value: 7,
                    },
                    Point {
                        x: 9,
                        y: 4,
                        value: 8,
                    },
                ],
            ],
            avg_per_row: vec![4, 6, 7, 7, 7],
        }
    }

    fn get_test_grid_object_two() -> Grid {
        Grid {
            data: vec![
                vec![
                    Point {
                        x: 0,
                        y: 0,
                        value: 9,
                    },
                    Point {
                        x: 1,
                        y: 0,
                        value: 9,
                    },
                    Point {
                        x: 2,
                        y: 0,
                        value: 9,
                    },
                ],
                vec![Point {
                    x: 1,
                    y: 1,
                    value: 9,
                }],
            ],
            avg_per_row: vec![4, 6, 7, 7, 7],
        }
    }
}
