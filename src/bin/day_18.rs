use std::collections::HashSet;

use regex::Regex;

use advent_2023::puzzle::Puzzle;
use advent_2023::twod::{Coord, Direction};

struct Step {
    dir: Direction,
    n: i32,

    #[allow(dead_code)]
    color: String,
}

fn a(steps: &Vec<Step>) -> u64 {
    let mut cur = Coord::new(0, 0);
    let mut history = HashSet::new();
    history.insert(cur);

    for &Step { dir, n, .. } in steps {
        for _ in 0..n {
            cur = cur.step(dir);
            history.insert(cur);
        }
    }

    let ymax = history.iter().map(|c| c.y).max().unwrap();
    let xmax = history.iter().map(|c| c.x).max().unwrap();

    let mut inside = 0;
    for x in 0..xmax {
        for y in 0..ymax {
            let c = Coord::new(x, y);

            if history.contains(&c) {
                inside += 1;
                continue;
            }
        }
    }

    inside
}

fn b(_steps: &Vec<Step>) -> u64 {
    0
}

fn main() {
    Puzzle {
        name: "18",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            text.into_iter()
                .map(|line| {
                    let re = Regex::new(r"([A-Z]) ([0-9]+) \(#([a-f0-9]{6})\)").unwrap();
                    let caps = re.captures(&line).unwrap();

                    let dir = match &caps[1] {
                        "U" => Direction::North,
                        "R" => Direction::East,
                        "D" => Direction::South,
                        "L" => Direction::West,
                        _ => panic!("unexpected direction"),
                    };

                    let n = caps[2].parse().unwrap();
                    let color = caps[3].to_string();

                    Step { dir, n, color }
                })
                .collect()
        },
    }
    .solve();
}
