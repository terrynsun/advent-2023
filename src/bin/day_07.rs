use std::collections::HashMap;

use advent_2023::puzzle::Puzzle;
use advent_2023::util::split_to_strings;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '1' => Card::V1,
            '2' => Card::V2,
            '3' => Card::V3,
            '4' => Card::V4,
            '5' => Card::V5,
            '6' => Card::V6,
            '7' => Card::V7,
            '8' => Card::V8,
            '9' => Card::V9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("invalid card character"),
        }
    }
}

// helper //
#[derive(Default)]
struct Counter {
    data: HashMap<Card, u64>,
}

impl Counter {
    fn insert(&mut self, val: Card) {
        self.data.entry(val).and_modify(|v| *v += 1).or_insert(1);
    }
}

// strongly ordered
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,     // 1     2  3 4 5
    OnePair,      // AA    1  2 3
    TwoPair,      // AA    QQ 1
    ThreeOfAKind, // AAA   1  2
    FullHouse,    // AAA   11
    FourOfAKind,  // AAAA  1
    FiveOfAKind,  // AAAAA
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        Self {
            cards: s.chars().map(Card::from_char).collect::<Vec<_>>()
        }
    }

    fn get_type(&self) -> HandType {
        let mut counter = Counter::default();

        self.cards.iter().for_each(|c| counter.insert(*c));

        let mut grouped = counter.data.into_iter().collect::<Vec<_>>();
        grouped.sort_by_key(|(_, v)| *v);

        match grouped.len() {
            // 12345
            5 => HandType::HighCard,
            // AA123
            4 => HandType::OnePair,
            3 => {
                if grouped[2].1 == 3 {
                    // AAA 1 2
                    HandType::ThreeOfAKind
                } else {
                    // AA QQ 1
                    HandType::TwoPair
                }
            },
            2 => {
                if grouped[1].1 == 4 {
                    // AAAA 1
                    HandType::FourOfAKind
                } else {
                    // AAA 11
                    HandType::FullHouse
                }
            },
            // AAAAA
            1 => HandType::FiveOfAKind,
            _ => panic!("only five cards in a set"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s_type = self.get_type();
        let o_type = other.get_type();
        if s_type != o_type {
            return s_type.partial_cmp(&o_type);
        }

        let zipped = self.cards.iter()
            .zip(other.cards.iter())
            .filter(|(a, b)| a != b).collect::<Vec<_>>();

        if let Some((a, b)) = zipped.get(0) {
            return Some(a.cmp(b))
        } else {
            return Some(std::cmp::Ordering::Equal)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Player {
    cards: Hand,
    bid: u64,
}

fn a(data: &Vec<Player>) -> u64 {
    let mut data = data.clone();
    data.sort();
    data.iter().enumerate().map(|(i, p)| ((i+1) as u64) * p.bid).sum()
}

fn main() {
    Puzzle {
        name: "7",
        parts: vec![a],
        delimiter: '\n',
        preprocess: |text| {
            text.iter().map(|line| {
                let parts = split_to_strings(line);
                Player {
                    cards: Hand::from_str(&parts[0]),
                    bid: parts[1].parse().unwrap(),
                }
            }).collect()
        },
    }.solve();
}
