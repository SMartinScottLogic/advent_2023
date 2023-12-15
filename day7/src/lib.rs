use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    hands: Vec<Hand>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut hands = self.hands.clone();
        let strength = "23456789TJQKA";
        hands.sort_by(|a, b| {
            let a_type = std::convert::Into::<Type>::into(a.hand.as_str());
            let b_type = std::convert::Into::<Type>::into(b.hand.as_str());
            let o = match a_type.partial_cmp(&b_type).unwrap() {
                std::cmp::Ordering::Equal => a
                    .hand
                    .chars()
                    .map(|c| (strength.find(c).unwrap()))
                    .cmp(b.hand.chars().map(|c| (strength.find(c).unwrap()))),
                c => c,
            };
            debug!(
                o = debug(o),
                a = debug(a),
                a_type = debug(&a_type),
                b = debug(b),
                b_type = debug(&b_type),
                "compare"
            );
            o
        });
        debug!(hands = debug(&hands), "sorted");
        let result = hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum();
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let strength = "J23456789TQKA";
        let mut hands = self
            .hands
            .iter()
            .map(|a| {
                let best_hand = Self::best_hand(strength, &a.hand);
                debug!(a = debug(a), best_hand, "best hand");
                (best_hand, a)
            })
            .collect::<Vec<_>>();
        hands.sort_by(|a, b| {
            let a_type = std::convert::Into::<Type>::into(a.0.as_str());
            let b_type = std::convert::Into::<Type>::into(b.0.as_str());
            let o = match a_type.partial_cmp(&b_type).unwrap() {
                std::cmp::Ordering::Equal => {
                    a.1.hand
                        .chars()
                        .map(|c| (strength.find(c).unwrap()))
                        .cmp(b.1.hand.chars().map(|c| (strength.find(c).unwrap())))
                }
                c => c,
            };
            debug!(
                o = debug(o),
                a = debug(a),
                a_type = debug(&a_type),
                b = debug(b),
                b_type = debug(&b_type),
                "compare"
            );
            o
        });
        debug!(hands = debug(&hands), "sorted");
        let result = hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.1.bid)
            .sum();
        Ok(result)
    }
}

impl Solution {
    fn compare_hands(strength: &str, a_hand: &str, b_hand: &str) -> std::cmp::Ordering {
        let a_type = std::convert::Into::<Type>::into(a_hand);
        let b_type = std::convert::Into::<Type>::into(b_hand);
        let o = match a_type.partial_cmp(&b_type).unwrap() {
            std::cmp::Ordering::Equal => a_hand
                .chars()
                .map(|c| (strength.find(c).unwrap()))
                .cmp(b_hand.chars().map(|c| (strength.find(c).unwrap()))),
            c => c,
        };
        debug!(
            o = debug(o),
            a_hand,
            a_type = debug(&a_type),
            b_hand,
            b_type = debug(&b_type),
            "compare"
        );
        o
    }
    fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }

    fn best_hand(strength: &str, hand: &str) -> String {
        let mut best_hand: Option<String> = None;
        let c = hand
            .chars()
            .fold(HashMap::<char, u64>::new(), |mut acc, v| {
                *acc.entry(v).or_default() += 1;
                acc
            });
        if c.get(&'J').is_none() {
            return hand.to_string();
        }
        for replacement in strength.chars() {
            if replacement == 'J' {
                continue;
            }
            let new_hand = hand.replacen('J', &replacement.to_string(), 1);
            match new_hand.find('J') {
                Some(_) => {
                    let inner_best_hand = Self::best_hand(strength, &new_hand);
                    let best = match best_hand {
                        None => inner_best_hand,
                        Some(hand)
                            if Self::compare_hands(strength, &inner_best_hand, &hand)
                                == std::cmp::Ordering::Greater =>
                        {
                            inner_best_hand
                        }
                        Some(hand) => hand,
                    };
                    best_hand = Some(best);
                }
                None => {
                    let best = match best_hand {
                        None => new_hand.clone(),
                        Some(hand)
                            if Self::compare_hands(strength, &new_hand, &hand)
                                == std::cmp::Ordering::Greater =>
                        {
                            new_hand.clone()
                        }
                        Some(hand) => hand,
                    };
                    best_hand = Some(best);
                }
            }
        }
        match best_hand {
            Some(v) => v,
            None => panic!("No best hand ?!?"),
        }
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            let hand: Hand = line.into();
            solution.add_hand(hand);
        }
        Ok(solution)
    }
}

#[derive(Debug, Clone)]
struct Hand {
    hand: String,
    bid: u64,
}
impl From<String> for Hand {
    fn from(value: String) -> Self {
        let r = regex::Regex::new(r"^(?<hand>\w+)\s+(?<bid>\d+)$").unwrap();
        let c = r.captures(&value).unwrap();
        let hand = c.name("hand").unwrap().as_str().to_string();
        let bid = c.name("bid").unwrap().as_str().parse().unwrap();
        Self { hand, bid }
    }
}
#[derive(Clone, Debug, PartialEq)]
enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}
impl PartialOrd<Type> for Type {
    fn partial_cmp(&self, other: &Type) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use Type::{Five, Four, FullHouse, HighCard, OnePair, Three, TwoPair};
        match (self, other) {
            (Five, Five) => Some(Equal),
            (Five, _) => Some(Greater),
            (_, Five) => Some(Less),

            (Four, Four) => Some(Equal),
            (Four, _) => Some(Greater),
            (_, Four) => Some(Less),

            (FullHouse, FullHouse) => Some(Equal),
            (FullHouse, _) => Some(Greater),
            (_, FullHouse) => Some(Less),

            (Three, Three) => Some(Equal),
            (Three, _) => Some(Greater),
            (_, Three) => Some(Less),

            (TwoPair, TwoPair) => Some(Equal),
            (TwoPair, _) => Some(Greater),
            (_, TwoPair) => Some(Less),

            (OnePair, OnePair) => Some(Equal),
            (OnePair, _) => Some(Greater),
            (_, OnePair) => Some(Less),

            (HighCard, HighCard) => Some(Equal),
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        let c = value
            .chars()
            .fold(HashMap::<char, u64>::new(), |mut acc, v| {
                *acc.entry(v).or_default() += 1;
                acc
            });
        debug!(value, c = debug(&c), "counts");
        match c.len() {
            1 => Self::Five,
            2 if c.values().any(|v| *v == 4) => Self::Four,
            2 if c.values().any(|v| *v == 3) => Self::FullHouse,
            3 if c.values().any(|v| *v == 2) => Self::TwoPair,
            3 if c.values().any(|v| *v == 3) => Self::Three,
            4 if c.values().any(|v| *v == 2) => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {}
