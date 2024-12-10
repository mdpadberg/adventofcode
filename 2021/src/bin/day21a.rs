use aoc2021::util::read_file_line_by_line_to_string;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("21"))
    );
}

fn solve_part_one(input: Vec<String>) -> u32 {
    let data = input
        .iter()
        .map(|line| line.split(":").last())
        .filter(|option| option.is_some())
        .map(|some| some.unwrap().trim())
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut dice = Dice::new();
    let mut player_one = Player {
        current_position: data[0],
        current_score: 0,
    };
    let mut player_two = Player {
        current_position: data[1],
        current_score: 0,
    };
    loop {
        if player_two.current_score >= 1000 {
            break player_one.current_score * dice.times
        } else {
            player_one.turn(dice.value);
            dice.turn();
        }
        if player_one.current_score >= 1000 {
            break player_two.current_score * dice.times
        } else {
            player_two.turn(dice.value);
            dice.turn();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dice {
    value: u32,
    times: u32,
}

impl Dice {
    fn new() -> Dice {
        Dice { value: 1, times: 0 }
    }

    fn turn(&mut self) {
        self.value = if (self.value + 3) > 100 {
            (self.value + 3) % 100
        } else {
            self.value + 3
        };
        self.times += 3;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    current_position: u32,
    current_score: u32,
}

impl Player {
    fn turn(&mut self, dice: u32) {
        let turn = dice + (dice + 1) + (dice + 2);
        let new_position = if (self.current_position + turn) % 10 == 0 {
            10
        } else {
            (self.current_position + turn) % 10
        };
        self.current_position = new_position;
        self.current_score += new_position;
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(solve_part_one(read_file_line_by_line_to_string_test("21-0")), 739785);
    }

    #[test]
    fn dice_turn() {
        let mut dice = Dice::new();
        assert_eq!(dice.value, 1);
        dice.turn();
        assert_eq!(dice.value, 4);

        let mut dice = Dice {
            value: 97,
            times: 0,
        };
        dice.turn();
        assert_eq!(dice.value, 100);
        let mut dice = Dice {
            value: 98,
            times: 0,
        };
        dice.turn();
        assert_eq!(dice.value, 1);
    }

    #[test]
    fn player_turn() {
        let mut player = Player {
            current_position: 4,
            current_score: 0,
        };
        player.turn(1);
        assert_eq!(
            Player {
                current_position: 10,
                current_score: 10
            },
            player
        );
        player.turn(7);
        assert_eq!(
            Player {
                current_position: 4,
                current_score: 14
            },
            player
        );
        player.turn(13);
        assert_eq!(
            Player {
                current_position: 6,
                current_score: 20
            },
            player
        );
        player.turn(19);
        assert_eq!(
            Player {
                current_position: 6,
                current_score: 26
            },
            player
        );
        let mut player = Player {
            current_position: 4,
            current_score: 990,
        };
        player.turn(91);
        assert_eq!(
            Player {
                current_position: 10,
                current_score: 1000
            },
            player
        );
    }
}
