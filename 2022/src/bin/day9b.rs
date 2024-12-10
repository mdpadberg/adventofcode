use std::collections::HashSet;

use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("9").unwrap()));
}

fn solve(input: String) -> usize {
    let directions: Vec<Direction> = input
        .split("\n")
        .flat_map(|line| Direction::parse(line))
        .collect();
    let mut rope: Vec<Thing> = vec![
        Thing {
            position: Coordinates { x: 0, y: 0 },
        };
        10
    ];
    let mut tail_visited: HashSet<Coordinates> =
        HashSet::from_iter(vec![Coordinates { x: 0, y: 0 }]);
    for direction in directions {
        let mut temp: Vec<Thing> = vec![];
        for (i, thing) in rope.iter_mut().enumerate() {
            if i == 0 {
                temp.push(thing.move_to(&direction));
            } else {
                temp.push(thing.follow(temp.get(i - 1).unwrap()))
            }
        }
        rope = temp;
        tail_visited.insert(rope[9].position);
    }
    tail_visited.len()
}

#[derive(Debug, Clone, Copy)]
struct Thing {
    position: Coordinates,
}

impl Thing {
    fn move_to(&mut self, direction: &Direction) -> Thing {
        Thing {
            position: self.position.move_to(direction),
        }
    }

    // T   = tail
    // .   = do nothing
    // A-Z = head position
    //  m a b c n
    //  d . . . e
    //  f . T . g
    //  h . . . i
    //  o j k l p
    fn follow(&mut self, thing: &Thing) -> Thing {
        match (
            thing.position.y - self.position.y,
            thing.position.x - self.position.x,
        ) {
            //a
            (2, -1) => Thing {
                position: self.position.move_to(&Direction::UP_AND_LEFT),
            },
            //b
            (2, 0) => Thing {
                position: self.position.move_to(&Direction::UP),
            },
            //c
            (2, 1) => Thing {
                position: self.position.move_to(&Direction::UP_AND_RIGHT),
            },
            //d
            (1, -2) => Thing {
                position: self.position.move_to(&Direction::UP_AND_LEFT),
            },
            //e
            (1, 2) => Thing {
                position: self.position.move_to(&Direction::UP_AND_RIGHT),
            },
            //f
            (0, -2) => Thing {
                position: self.position.move_to(&Direction::LEFT),
            },
            //g
            (0, 2) => Thing {
                position: self.position.move_to(&Direction::RIGHT),
            },
            //h
            (-1, -2) => Thing {
                position: self.position.move_to(&Direction::DOWN_AND_LEFT),
            },
            //i
            (-1, 2) => Thing {
                position: self.position.move_to(&Direction::DOWN_AND_RIGHT),
            },
            //j
            (-2, -1) => Thing {
                position: self.position.move_to(&Direction::DOWN_AND_LEFT),
            },
            //k
            (-2, 0) => Thing {
                position: self.position.move_to(&Direction::DOWN),
            },
            //l
            (-2, 1) => Thing {
                position: self.position.move_to(&Direction::DOWN_AND_RIGHT),
            },
            //m
            (2, -2) => Thing {
                position: self.position.move_to(&Direction::UP_AND_LEFT),
            },
            //n
            (2, 2) => Thing {
                position: self.position.move_to(&Direction::UP_AND_RIGHT),
            },
            //o
            (-2, -2) => Thing {
                position: self.position.move_to(&Direction::DOWN_AND_LEFT),
            },
            //p
            (-2, 2) => Thing {
                position: self.position.move_to(&Direction::DOWN_AND_RIGHT),
            },
            //dont move
            _ => Thing {
                position: self.position,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UP_AND_LEFT,
    UP_AND_RIGHT,
    DOWN_AND_LEFT,
    DOWN_AND_RIGHT,
}

impl Direction {
    fn parse(input: &str) -> Vec<Direction> {
        match input.split_ascii_whitespace().collect_tuple() {
            Some(("U", i)) => vec![Direction::UP; i.parse::<usize>().unwrap()],
            Some(("D", i)) => vec![Direction::DOWN; i.parse::<usize>().unwrap()],
            Some(("R", i)) => vec![Direction::RIGHT; i.parse::<usize>().unwrap()],
            Some(("L", i)) => vec![Direction::LEFT; i.parse::<usize>().unwrap()],
            _ => panic!("AAAAAAAAAAAAAH!"),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn move_to(&self, direction: &Direction) -> Coordinates {
        match direction {
            Direction::UP => Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            Direction::DOWN => Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Direction::LEFT => Coordinates {
                x: self.x - 1,
                y: self.y,
            },
            Direction::RIGHT => Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            Direction::UP_AND_LEFT => Coordinates {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::UP_AND_RIGHT => Coordinates {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::DOWN_AND_LEFT => Coordinates {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::DOWN_AND_RIGHT => Coordinates {
                x: self.x + 1,
                y: self.y - 1,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use aoc2022::util::read_test_data_for_day;
    use super::*;

    #[test]
    fn solvetest() {
        assert_eq!(36, solve(read_test_data_for_day("9-7").unwrap()));
    }
}
