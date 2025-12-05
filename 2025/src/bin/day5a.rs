use aoc2025::util::read_data_for_day;
use itertools::Itertools;
use std::ops::{Range, RangeInclusive};
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let result = solve(read_data_for_day("5").ok_or(MyError::ParsingInputDataError)?)?;
    println!("{result}");
    Ok(())
}

fn solve(input: String) -> Result<usize, MyError> {
    let (fresh_ingredient_id_ranges, available_ingredient_ids) = parse(input)?;
    let answer: usize = available_ingredient_ids
        .ids
        .iter()
        .filter(|available_ingredient_id| {
            fresh_ingredient_id_ranges
                .ranges
                .iter()
                .any(|range| range.contains(available_ingredient_id))
        })
        .count();
    Ok(answer)
}

fn parse(input: String) -> Result<(FreshIngredientIdRanges, AvailableIngredientIds), MyError> {
    let (first, second) = input
        .split_once("\n\n")
        .ok_or(MyError::ParsingInputDataError)?;
    let fresh_ingredient_id_ranges = FreshIngredientIdRanges {
        ranges: first
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line: &str| Ok(line.split_once("-").ok_or(MyError::ParsingInputDataError)?))
            .map(|split: Result<(&str, &str), MyError>| {
                let split = split?;
                let begin: u64 = split.0.parse()?;
                let end: u64 = split.1.parse()?;
                let range: FreshIngredientIdRange = begin..=end;
                Ok::<FreshIngredientIdRange, MyError>(range)
            })
            .collect::<Result<Vec<FreshIngredientIdRange>, MyError>>()?,
    };
    let available_ingredient_ids = AvailableIngredientIds {
        ids: second
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line: &str| Ok(line.parse()?))
            .collect::<Result<Vec<IngredientId>, MyError>>()?,
    };
    Ok((fresh_ingredient_id_ranges, available_ingredient_ids))
}

#[derive(Debug)]
struct FreshIngredientIdRanges {
    ranges: Vec<FreshIngredientIdRange>,
}

type FreshIngredientIdRange = RangeInclusive<u64>;

#[derive(Debug)]
struct AvailableIngredientIds {
    ids: Vec<IngredientId>,
}

type IngredientId = u64;

#[derive(Debug, Error, Clone)]
enum MyError {
    #[error("ParsingInputDataError")]
    ParsingInputDataError,
    #[error("ParseIntError")]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2025::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(3, solve(read_test_data_for_day("5-0").unwrap()).unwrap());
    }
}
