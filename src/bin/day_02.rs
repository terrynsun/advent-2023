use advent_2023::puzzle::*;

use regex::Regex;

use std::collections::HashMap;

struct Game {
    n: i32,
    draws: Vec<HashMap<String, i32>>, // color -> count
}

fn two_a(data: &Vec<Game>) -> i32 {
    let max: HashMap<String, i32> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    data.iter()
        .filter(|g| {
            g.draws
                .iter()
                .all(|d| d.iter().all(|(color, n)| n <= &max[color]))
        })
        .map(|g| g.n)
        .sum()
}

fn two_b(data: &Vec<Game>) -> i32 {
    data.iter()
        .map(|g| {
            let mut highest = HashMap::new();
            for draw in g.draws.iter() {
                for (color, n) in draw.iter() {
                    highest
                        .entry(color)
                        .and_modify(|x| *x = std::cmp::max(*x, *n))
                        .or_insert(*n);
                }
            }
            highest
        })
        .map(|highest| highest.iter().fold(1, |acc, (_, n)| acc * n))
        .sum()
}

fn main() {
    Puzzle {
        name: "2",
        parts: vec![two_a, two_b],
        delimiter: '\n',
        preprocess: |text| {
            text.iter()
                .map(|line| {
                    let game_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
                    let captures = game_re.captures(line).unwrap();

                    let draws = captures[2]
                        .split(';')
                        .map(|d| {
                            let balls_re = Regex::new(r"([0-9]+) ([a-z]+)").unwrap();
                            let mut draw = HashMap::new();

                            for (_, [i, color]) in balls_re.captures_iter(d).map(|c| c.extract()) {
                                draw.insert(color.to_string(), i.parse::<i32>().unwrap());
                            }

                            draw
                        })
                        .collect();

                    Game {
                        n: captures[1].parse().unwrap(),
                        draws,
                    }
                })
                .collect()
        },
    }
    .solve();
}
