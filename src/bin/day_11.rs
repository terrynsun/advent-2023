use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;
use advent_2023::twod::Coord;

fn pairwise_distance(data: &Vec<Coord>) -> u64 {
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

fn expand(mut galaxies: Vec<Coord>, expansion_factor: i32) -> Vec<Coord> {
    let mut xset = HashSet::new();
    let mut yset = HashSet::new();

    for Coord { x, y } in galaxies.iter() {
        xset.insert(*x);
        yset.insert(*y);
    }

    let xmax = *xset.iter().max().unwrap();
    let ymax = *yset.iter().max().unwrap();

    // Reverse so we don't need to also expand the coords of the empty rows
    for x in (0..xmax).rev() {
        if !xset.contains(&x) {
            galaxies.iter_mut().for_each(|c| {
                if c.x > x {
                    c.x += expansion_factor;
                }
            });
        }
    }

    for y in (0..ymax).rev() {
        if !yset.contains(&y) {
            galaxies.iter_mut().for_each(|c| {
                if c.y > y {
                    c.y += expansion_factor;
                }
            });
        }
    }

    galaxies
}

fn a(data: &Vec<Coord>) -> u64 {
    let data = expand(data.clone(), 1);
    pairwise_distance(&data)
}

fn b(data: &Vec<Coord>) -> u64 {
    let data = expand(data.clone(), 1000000-1);
    pairwise_distance(&data)
}

fn main() {
    Puzzle {
        name: "11",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            let mut galaxies = Vec::new();

            // okay, I guess this is more readable as a nested array.
            for (y, line) in text.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        galaxies.push(Coord::from_usize(x, y));
                    }
                }
            }

            galaxies
        },
    }.solve();
}
