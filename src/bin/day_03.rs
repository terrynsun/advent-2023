use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;

struct Engine {
    pub data: Vec<Vec<char>>,
    // These should be usize but it's annoying to deal with underflows.
    pub xmax: i32,
    pub ymax: i32,
}

impl Engine {
    fn new(data: Vec<Vec<char>>) -> Self {
        let ymax = data.len() as i32;
        let xmax = data[0].len() as i32;

        Self { data, ymax, xmax }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.xmax && y < self.ymax
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if !self.in_bounds(x, y) {
            None
        } else {
            Some(self.data[y as usize][x as usize])
        }
    }

    pub fn is_symbol(&self, x: i32, y: i32) -> bool {
        self.get(x, y)
            .map_or(false, |c| !c.is_ascii_digit() && c != '.')
    }

    pub fn try_get_digit(&self, x: i32, y: i32) -> Option<u32> {
        self.get(x, y).map(|c| c.to_digit(10)).flatten()
    }

    /// Returns the number and the coordinates of its firs digit.
    pub fn try_get_full_part_number(&self, x: i32, y: i32) -> Option<(u32, i32, i32)> {
        let mut digits = vec![];
        if self.try_get_digit(x, y).is_none() {
            return None;
        }

        let mut least_x = x;

        for new_x in (0..x).rev() {
            if let Some(n) = self.try_get_digit(new_x, y) {
                digits.push(n);
                least_x = std::cmp::min(least_x, new_x);
            } else {
                break;
            }
        }

        digits.reverse();

        for new_x in x..self.xmax {
            if let Some(n) = self.try_get_digit(new_x, y) {
                digits.push(n);
            } else {
                break;
            }
        }

        Some((Self::construct(digits), least_x, y))
    }

    fn construct(digits: Vec<u32>) -> u32 {
        digits.iter().fold(0, |acc, e| acc * 10 + e)
    }

    fn find_adjacents(&self, x: i32, y: i32) -> HashSet<(u32, i32, i32)> {
        let mut set = HashSet::new();

        for (x_diff, y_diff) in
                [
                    (-1, -1), (0, -1), (1, -1),
                    (-1,  0),          (1,  0),
                    (-1,  1), (0,  1), (1,  1),
                ].iter() {

            if let Some(n) = self.try_get_full_part_number(x + x_diff, y + y_diff) {
                set.insert(n);
            }
        }

        set
    }
}

fn a(engine: &Engine) -> i32 {
    let mut set = HashSet::new();

    for y in 0..engine.ymax {
        for x in 0..engine.xmax {
            if !engine.is_symbol(x, y) {
                continue;
            }

            for v in engine.find_adjacents(x, y) {
                set.insert(v);
            }
        }
    }

    set.iter().map(|(x, _, _)| x).sum::<u32>() as i32
}

fn b(engine: &Engine) -> i32 {
    let mut total = 0;

    for y in 0..engine.ymax {
        for x in 0..engine.xmax {
            if !engine.is_symbol(x, y) {
                continue;
            }

            let adjs: Vec<u32> = engine.find_adjacents(x, y).into_iter().map(|(x, _, _)| x).collect();
            if adjs.len() > 1 {
                total += adjs.iter().fold(1, |acc, e| acc * e);
            }
        }
    }

    total as i32
}

fn main() {
    Puzzle {
        name: "3",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| Engine::new(text.into_iter().map(|line| line.chars().collect()).collect()),
    }
    .solve();
}
