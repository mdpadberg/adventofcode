use aoc2025::util::read_data_for_day;
use itertools::Itertools;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let result = solve(read_data_for_day("3").ok_or(MyError::ParsingInputDataError)?)?;
    println!("{result}");
    Ok(())
}

fn solve(input: String) -> Result<u64, MyError> {
    let parsed = parse(input)?;
    let mut result: u64 = 0;
    for bank in parsed {
        let max_first_number =
            max_first_number(&bank).ok_or(MyError::CannotFindFirstNumberError)?;
        let batteries_after_max_first_number =
            bank.batteries.iter().skip(max_first_number.position + 1);
        let max_second_number = batteries_after_max_first_number
            .max_by_key(|battery| battery.joltage)
            .ok_or(MyError::CannotFindSecondNumberError)?;
        let largest_joltage_possible: u64 =
            format!("{}{}", max_first_number.joltage, max_second_number.joltage).parse()?;
        result += largest_joltage_possible;
    }
    Ok(result)
}

fn max_first_number(bank: &Bank) -> Option<Battery> {
    let mut sorted_by_joltage = bank
        .batteries
        .iter()
        .sorted_by(|a, b| {
            // We need this because we want to sort the highest joltage last, but if we have
            // two batteries with the same joltage, the one with the lowest position should
            // go first (because we want the most left highest number)
            if a.joltage == b.joltage {
                b.position.cmp(&a.position)
            } else {
                a.joltage.cmp(&b.joltage)
            }
        })
        .rev();
    let length = &sorted_by_joltage.len();
    let mut max_first_number: Option<Battery> = None;
    while let Some(highest_joltage_in_batteries) = sorted_by_joltage.next() {
        // Highest Nummber cannot be the last in the row, because else its not possible to find a 2 number answer
        // For example: 811111111111119
        // 89 is the higest number you can get here
        let higest_number_is_last_in_row = highest_joltage_in_batteries.position == (length - 1);
        if !higest_number_is_last_in_row {
            max_first_number = Some(highest_joltage_in_batteries.clone());
            break;
        }
    }
    max_first_number
}

fn parse(input: String) -> Result<Vec<Bank>, MyError> {
    let mut result = vec![];
    for line in input.split("\n") {
        let batteries = line
            .chars()
            .enumerate()
            .map(|(i, char)| {
                Ok(Battery {
                    position: i,
                    joltage: char.to_digit(10).ok_or(MyError::ParsingInputDataError)?,
                })
            })
            .collect::<Result<Vec<Battery>, MyError>>()?;
        result.push(Bank {
            batteries: batteries,
        });
    }
    Ok(result)
}

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<Battery>,
}

#[derive(Debug, Clone)]
struct Battery {
    position: usize,
    joltage: u32,
}

#[derive(Debug, Error)]
enum MyError {
    #[error("ParseIntError")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParsingInputDataError")]
    ParsingInputDataError,
    #[error("CannotFindMaxError")]
    CannotFindFirstNumberError,
    #[error("CannotFindSecondNumberError")]
    CannotFindSecondNumberError,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2025::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(357, solve(read_test_data_for_day("3-1").unwrap()).unwrap());
    }
}
