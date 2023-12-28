use std::collections::{HashMap, HashSet};

use advent_2023::puzzle::Puzzle;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn from_str(x: &str, y: &str, z: &str) -> Self {
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }

    pub fn from_usize(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
            z: z as i32,
        }
    }

    pub fn above(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Brick {
    a: Coord,
    b: Coord,
    name: char,
}

impl Brick {
    fn fall(&self) -> Self {
        Brick {
            a: self.a.below(),
            b: self.b.below(),
            name: self.name,
        }
    }
}

impl IntoIterator for Brick {
    type Item = Coord;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = vec![];

        for x in self.a.x..=self.b.x {
            for y in self.a.y..=self.b.y {
                for z in self.a.z..=self.b.z {
                    vec.push(Coord::new(x, y, z));
                }
            }
        }

        vec.into_iter()
    }
}

#[derive(Debug, Default)]
struct Space {
    data: HashMap<Coord, char>,
    bricks_by_name: HashMap<char, Brick>,

    xmax: i32,
    ymax: i32,
    zmax: i32,
}

impl Space {
    #[allow(dead_code)]
    fn print_view_from_x(&self) {
        println!();
        for z in (1..=self.zmax).rev() {
            print!("{z}: ");
            for x in 0..=self.xmax {
                let mut c = '.';
                for y in 0..=self.ymax {
                    if let Some(brick) = self.search(Coord::new(x, y, z)) {
                        c = brick;
                    }
                }
                print!("{c}");
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_view_from_y(&self) {
        println!();
        for z in (1..=self.zmax).rev() {
            print!("{z}: ");
            for y in 0..=self.ymax {
                let mut c = '.';
                for x in 0..=self.xmax {
                    if let Some(brick) = self.search(Coord::new(x, y, z)) {
                        c = brick;
                    }
                }
                print!("{c}");
            }
            println!();
        }
    }

    fn insert(&mut self, brick: Brick) {
        self.bricks_by_name.insert(brick.name, brick);

        for c in brick.into_iter() {
            self.data.insert(c, brick.name);

            self.xmax = std::cmp::max(self.xmax, c.x);
            self.ymax = std::cmp::max(self.ymax, c.y);
            self.zmax = std::cmp::max(self.zmax, c.z);
        }
    }

    fn find_brick(&self, c: char) -> Option<Brick> {
        self.bricks_by_name.get(&c).copied()
    }

    fn remove(&mut self, brick: &Brick) {
        for c in brick.into_iter() {
            // does not check if removing only the right brick (assumes no intersections, I guess)
            self.data.remove(&c);
        }
        // does not fix {x,y,z}-max
    }

    fn remove_by_name(&mut self, name: char) {
        self.remove(&self.find_brick(name).unwrap())
    }

    fn search(&self, c: Coord) -> Option<char> {
        self.data.get(&c).cloned()
    }

    fn find_above(&self, brick: Brick) -> Vec<char> {
        let mut aboves = HashSet::new();

        for c in brick.into_iter() {
            if let Some(name) = self.search(c.above()) {
                // ignore bricks that are below themselves because they are than 1 unit high
                if name == brick.name {
                    continue;
                }

                aboves.insert(name);
            }
        }

        Vec::from_iter(aboves)
    }

    fn find_below(&self, brick: &Brick) -> Vec<char> {
        let mut below = HashSet::new();

        for c in brick.into_iter() {
            if let Some(name) = self.search(c.below()) {
                // ignore bricks that are above themselves because they are than 1 unit high
                if name == brick.name {
                    continue;
                }

                below.insert(name);
            }
        }

        Vec::from_iter(below)
    }

    // Simulate bricks falling until no bricks fall
    fn drop_all_bricks(&mut self) {
        loop {
            let mut new_bricks = vec![];
            for (_name, brick) in self.bricks_by_name.iter() {
                // todo - use while let?
                if let Some(new_brick) = self.drop_brick(brick) {
                    new_bricks.push(new_brick);
                }
            }

            if new_bricks.is_empty() {
                break;
            }

            new_bricks.into_iter().for_each(|b| {
                self.remove_by_name(b.name);
                self.insert(b);
            });
        }
    }

    fn drop_brick(&self, brick: &Brick) -> Option<Brick> {
        if !self.find_below(brick).is_empty() {
            return None;
        }

        // cannot fall into the floor (z=0)
        if brick.a.z == 1 || brick.b.z == 1 {
            return None;
        }

        let new_brick = brick.fall();

        Some(new_brick)
    }

    fn is_fully_supported_by(&self, brick: &Brick, others: Vec<Brick>) -> bool {
        let bricks_below = self.find_below(brick);
        bricks_below
            .iter()
            .map(|&name| self.find_brick(name).unwrap())
            .all(|below| others.contains(&below))
    }
}

fn a(bricks: &Vec<Brick>) -> usize {
    let mut space: Space = Default::default();

    for &b in bricks.iter() {
        space.insert(b);
    }

    space.drop_all_bricks();

    // You can disintegrate a brick when all bricks above it are above 2+ bricks.
    space
        .bricks_by_name
        .iter()
        .filter(|(&_c, &b)| {
            let bricks_above = space.find_above(b);
            if bricks_above.is_empty() {
                return true;
            }

            !bricks_above.iter().any(|above| {
                let brick = space.find_brick(*above).unwrap();
                space.is_fully_supported_by(&brick, vec![b])
            })
        })
        .count()
}

fn b(_data: &Vec<Brick>) -> usize {
    0
}

fn main() {
    Puzzle {
        name: "22",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            let mut cur_name = '@'; // @ is 1 before A
            text.iter()
                .map(|line| {
                    let re = Regex::new(r"([0-9]+),([0-9]+),([0-9]+)~([0-9]+),([0-9]+),([0-9]+)")
                        .unwrap();
                    let (_full, [x1, y1, z1, x2, y2, z2]) = re.captures(line).unwrap().extract();
                    cur_name = incr_char(cur_name);
                    Brick {
                        a: Coord::from_str(x1, y1, z1),
                        b: Coord::from_str(x2, y2, z2),
                        name: cur_name,
                    }
                })
                .collect()
        },
    }
    .solve();
}

fn incr_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}
