use std::cmp::Ordering;

use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("7").unwrap()));
}

fn solve(input: String) -> usize {
    input
        .split("\n")
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.trim())
                .collect_tuple::<(&str, &str)>()
                .unwrap();
            let cards = values.0.to_string();
            Hand {
                score: score(Pair::how_many_pairs(&cards)),
                cards,
                bid: values.1.parse::<usize>().unwrap(),
            }
        })
        .sorted()
        .enumerate()
        .map(|(i,hand)| (i + 1) * hand.bid)
        .sum::<usize>()
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    score: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            //If two hands have the same type, a second ordering rule takes effect. 
            // Start by comparing the first card in each hand. If these cards are different, 
            // the hand with the stronger first card is considered stronger. If the first card in 
            // each hand have the same label, however, then move on to considering the second card 
            // in each hand.
            Ordering::Equal => {
                let possible_cards = "23456789TJQKA";
                for (a, b) in self.cards.chars().zip(other.cards.chars()) {
                    if possible_cards.find(a).unwrap() > possible_cards.find(b).unwrap() {
                        return Ordering::Greater;
                    }
                    if possible_cards.find(a).unwrap() < possible_cards.find(b).unwrap() {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (&self.bid, &self.cards, &self.score) == (&other.bid, &other.cards, &other.score)
    }
}

impl Eq for Hand {}

#[derive(Debug, PartialEq)]
struct Pair {
    card_count: usize,
    card: char,
}

impl Pair {
    // Exmples:
    // "32T3K" -> [Pair { card_count: 1, card: '2' }, Pair { card_count: 2, card: '3' }, Pair { card_count: 1, card: 'K' }, Pair { card_count: 1, card: 'T' }]
    // "T55J5" -> [Pair { card_count: 3, card: '5' }, Pair { card_count: 1, card: 'J' }, Pair { card_count: 1, card: 'T' }]
    // "KK677" -> [Pair { card_count: 1, card: '6' }, Pair { card_count: 2, card: '7' }, Pair { card_count: 2, card: 'K' }]
    // "KTJJT" -> [Pair { card_count: 2, card: 'J' }, Pair { card_count: 1, card: 'K' }, Pair { card_count: 2, card: 'T' }]
    // "QQQJA" -> [Pair { card_count: 1, card: 'A' }, Pair { card_count: 1, card: 'J' }, Pair { card_count: 3, card: 'Q' }]
    fn how_many_pairs(cards: &String) -> Vec<Pair> {
        cards
            .chars()
            .sorted()
            .dedup_with_count()
            .map(|(card_count, card)| Pair { card_count, card })
            .collect::<Vec<Pair>>()
    }
}

fn score(pairs: Vec<Pair>) -> usize {
    // Examples:
    // "32T3K" -> 7  -> (1*1) + (2*2) + (1*1) + (1*1) -> [Pair { card_count: 1, card: '2' }, Pair { card_count: 2, card: '3' }, Pair { card_count: 1, card: 'K' }, Pair { card_count: 1, card: 'T' }]
    // "T55J5" -> 11 -> (3*3) + (1*1) + (1*1)         -> [Pair { card_count: 3, card: '5' }, Pair { card_count: 1, card: 'J' }, Pair { card_count: 1, card: 'T' }]
    // "KK677" -> 9  -> (1*1) + (2*2) + (2*2)         -> [Pair { card_count: 1, card: '6' }, Pair { card_count: 2, card: '7' }, Pair { card_count: 2, card: 'K' }]
    // "KTJJT" -> 9  -> (2*2) + (1*1) + (2*2)         -> [Pair { card_count: 2, card: 'J' }, Pair { card_count: 1, card: 'K' }, Pair { card_count: 2, card: 'T' }]
    // "QQQJA" -> 11 -> (1*1) + (1*1) + (3*3)         -> [Pair { card_count: 1, card: 'A' }, Pair { card_count: 1, card: 'J' }, Pair { card_count: 3, card: 'Q' }]
    pairs
        .iter()
        .map(|pair| pair.card_count * pair.card_count)
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(6440, solve(read_test_data_for_day("7-0").unwrap()));
    }
}
