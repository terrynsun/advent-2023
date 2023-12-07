use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;

#[derive(Default)]
struct Card {
    // winning numbers
    pub left: HashSet<u32>,

    // the ones we have
    pub right: HashSet<u32>,
}

impl Card {
    pub fn win_count(&self) -> i32 {
        self.left.intersection(&self.right).count() as i32
    }

    pub fn score(n: i32) -> i32 {
        2i32.pow((n - 1) as u32)
    }
}

fn a(cards: &Vec<Card>) -> i32 {
    cards
        .iter()
        .map(Card::win_count)
        .filter(|size| *size > 0)
        .map(Card::score)
        .sum()
}

fn b(cards: &Vec<Card>) -> i32 {
    let winning_count: Vec<_> = cards.iter().map(Card::win_count).collect();

    let mut card_counts = vec![1; cards.len()];

    for i in 0..cards.len() {
        let copies = winning_count[i];
        if copies == 0 {
            continue;
        }

        for j in 0..copies {
            card_counts[i + (j as usize) + 1] += card_counts[i];
        }
    }

    card_counts.iter().sum()
}

fn main() {
    Puzzle {
        name: "4",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            text.iter()
                .map(|line| {
                    // Remove "Card n:"
                    let split_card: Vec<&str> = line.split(": ").collect();

                    // Split on |
                    let split_nums: Vec<&str> = split_card[1].clone().split(" | ").collect();

                    // Parse nums
                    let left = split_nums[0]
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    let right = split_nums[1]
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    Card { left, right }
                })
                .collect()
        },
    }
    .solve();
}
