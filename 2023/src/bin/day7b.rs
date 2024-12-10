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
        .map(|(i, hand)| (i + 1) * hand.bid)
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
                let possible_cards = "J23456789TQKA";
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Pair {
    card_count: usize,
    card: char,
}

impl Pair {
    // Exmples:
    // "32T3K" -> [Pair { card_count: 1, card: '2' }, Pair { card_count: 2, card: '3' }, Pair { card_count: 1, card: 'K' }, Pair { card_count: 1, card: 'T' }]
    // "T55J5" -> [Pair { card_count: 4, card: '5' }, Pair { card_count: 1, card: 'T' }]
    // "KK677" -> [Pair { card_count: 1, card: '6' }, Pair { card_count: 2, card: '7' }, Pair { card_count: 2, card: 'K' }]
    // "KTJJT" -> [Pair { card_count: 1, card: 'K' }, Pair { card_count: 4, card: 'T' }]
    // "QQQJA" -> [Pair { card_count: 1, card: 'A' }, Pair { card_count: 4, card: 'Q' }]
    fn how_many_pairs(cards: &String) -> Vec<Pair> {
        let pairs = cards
            .chars()
            .sorted()
            .dedup_with_count()
            .map(|(card_count, card)| Pair { card_count, card })
            .collect::<Vec<Pair>>();
        if !cards.to_uppercase().contains("J") {
            pairs
        } else {
            // contains joker and plus amount of jokers to highest card count pair
            if let Some(higest_pair) = pairs
                .iter()
                .filter(|pair| pair.card != 'J')
                .max_by_key(|pair| pair.card_count)
            {
                let amount_of_jokers = pairs
                    .iter()
                    .filter(|pair| pair.card == 'J')
                    .map(|pair| pair.card_count)
                    .sum::<usize>();
                let new_highest_pair = Pair {
                    card_count: higest_pair.card_count + amount_of_jokers,
                    card: higest_pair.card,
                };
                let mut new_pairs = pairs
                    .iter()
                    .filter(|pair| pair.card != 'J' && pair != &higest_pair)
                    .map(|pair| pair.to_owned())
                    .collect::<Vec<Pair>>();
                new_pairs.push(new_highest_pair);
                new_pairs
            } else {
                // If there is not highest pair then JJJJJ, because thats what we filter on line 100
                vec![Pair {
                    card_count: 5,
                    card: 'J',
                }]
            }
        }
    }
}

fn score(pairs: Vec<Pair>) -> usize {
    // Examples:
    // "32T3K" -> 7  -> (1*1) + (2*2) + (1*1) + (1*1) -> [Pair { card_count: 1, card: '2' }, Pair { card_count: 2, card: '3' }, Pair { card_count: 1, card: 'K' }, Pair { card_count: 1, card: 'T' }]
    // "T55J5" -> 17 -> (1*1) + (4*4)                 -> [Pair { card_count: 1, card: 'T' }, Pair { card_count: 4, card: '5' }]
    // "KK677" -> 9  -> (1*1) + (2*2) + (2*2)         -> [Pair { card_count: 1, card: '6' }, Pair { card_count: 2, card: '7' }, Pair { card_count: 2, card: 'K' }]
    // "KTJJT" -> 17 -> (1*1) + (4*4)                 -> [Pair { card_count: 1, card: 'K' }, Pair { card_count: 4, card: 'T' }]
    // "QQQJA" -> 17 -> (1*1) + (4*4)                 -> [Pair { card_count: 1, card: 'A' }, Pair { card_count: 4, card: 'Q' }]
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
        assert_eq!(5905, solve(read_test_data_for_day("7-0").unwrap()));
    }
}
