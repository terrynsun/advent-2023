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
        for c in brick.into_iter() {
            self.data.insert(c, brick.name);

            self.xmax = std::cmp::max(self.xmax, c.x);
            self.ymax = std::cmp::max(self.ymax, c.y);
            self.zmax = std::cmp::max(self.zmax, c.z);
        }
    }

    fn remove(&mut self, brick: &Brick) {
        for c in brick.into_iter() {
            // does not check if removing only the right brick.
            self.data.remove(&c);

        }
        // todo does not fix {x,y,z}-max
    }

    fn search(&self, c: Coord) -> Option<char> {
        self.data.get(&c).cloned()
    }

    fn find_above(&self, brick: Brick) -> Vec<char> {
        let mut aboves = HashSet::new();

        for c in brick.into_iter() {
            if let Some(name) = self.search(c.above()) {
                // ignore bricks that are more than 1 unit high (and thus are under themselves...)
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
                // ignore bricks that are more than 1 unit high (and thus are under themselves...)
                if name == brick.name {
                    continue;
                }

                below.insert(name);
            }
        }

        Vec::from_iter(below)
    }

    fn fall(&mut self, brick: &Brick) -> Option<Brick> {
        if !self.find_below(brick).is_empty() {
            return None;
        }

        // cannot fall into the floor (z=0)
        if brick.a.z == 1 || brick.b.z == 1 {
            return None;
        }

        self.remove(brick);

        let new_brick = brick.fall();
        self.insert(new_brick);

        Some(new_brick)
    }
}

fn a(bricks: &Vec<Brick>) -> u64 {
    let mut space: Space = Default::default();
    let mut bricks_by_name = HashMap::new();

    for &b in bricks.iter() {
        bricks_by_name.insert(b.name, b);
        space.insert(b);
    }

    // Bricks fall
    loop {
        let mut has_fallen = false;
        for (_name, brick) in bricks_by_name.iter_mut() {
            if let Some(new_brick) = space.fall(brick) {
                *brick = new_brick;
                has_fallen = true;
            }
        }

        if !has_fallen {
            break;
        }
    }

    // You can disintegrate a brick when all bricks above it are above 2+ bricks.
    let mut count = 0;
    for (&_c, &b) in bricks_by_name.iter() {
        let bricks_above = space.find_above(b);
        if bricks_above.is_empty() {
            count += 1;
            continue;
        }

        let mut is_sole_support = false;
        for above in bricks_above {
            let above_brick = bricks_by_name.get(&above).unwrap();
            let belows = space.find_below(above_brick);
            if belows.len() == 1 {
                is_sole_support = true;
            }
        }

        if !is_sole_support {
            count += 1;
        }
    }

    count
}

fn b(_data: &Vec<Brick>) -> u64 {
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
