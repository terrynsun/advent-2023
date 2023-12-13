use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord {
            x: x as usize,
            y: y as usize
        }
    }
}

fn a(data: &Vec<Coord>) -> u64 {
    let mut sum = 0;
    for i in 0..data.len() {
        for j in i..data.len() {
            let g1 = data[i];
            let g2 = data[j];

            let x = (g1.x as i64) - (g2.x as i64);
            let y = (g1.y as i64) - (g2.y as i64);
            sum += x.abs() + y.abs();
        }
    }

    sum as u64
}

fn b(_data: &Vec<Coord>) -> u64 {
    0
}

fn main() {
    Puzzle {
        name: "11",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            let mut galaxies = Vec::new();
            let mut xset = HashSet::new();
            let mut yset = HashSet::new();

            // okay, I guess this is more readable as a nested array.
            for (y, line) in text.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        galaxies.push(Coord::new(x, y));
                        xset.insert(x);
                        yset.insert(y);
                    }
                }
            }

            // Expansion
            let xmax = *xset.iter().max().unwrap();
            let ymax = *yset.iter().max().unwrap();

            // Reverse so we don't need to also expand the coords of the empty rows
            for x in (0..xmax).rev() {
                if !xset.contains(&x) {
                    galaxies.iter_mut().for_each(|c| {
                        if c.x > x {
                            c.x += 1000000-1;
                        }
                    });
                }
            }

            for y in (0..ymax).rev() {
                if !yset.contains(&y) {
                    galaxies.iter_mut().for_each(|c| {
                        if c.y > y {
                            c.y += 1000000-1;
                        }
                    });
                }
            }

            galaxies
        },
    }.solve();
}
