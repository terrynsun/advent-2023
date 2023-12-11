use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn pretty(&self) -> String {
        format!("({},{})", self.x, self.y)
    }

    fn north(&self) -> Coord {
        Coord { x: self.x, y: self.y-1 }
    }

    fn south(&self) -> Coord {
        Coord { x: self.x, y: self.y+1 }
    }

    fn west(&self) -> Coord {
        Coord { x: self.x-1, y: self.y }
    }

    fn east(&self) -> Coord {
        Coord { x: self.x+1, y: self.y }
    }
}

#[derive(Debug)]
struct Map {
    pub data: Vec<Vec<char>>,

    pub start: Coord,

    // These should be usize but it's annoying to deal with underflows.
    pub xmax: i32,
    pub ymax: i32,
}

impl Map {
    fn new(data: Vec<Vec<char>>) -> Self {
        let ymax = data.len() as i32;
        let xmax = data[0].len() as i32;

        let start = Self::find_start(&data);

        Self { data, start, ymax, xmax }
    }

    fn empty(xmax: i32, ymax: i32) -> Self {
        let data = (0..ymax)
            .map(|_| (0..xmax).map(|_| '.').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self::new(data)
    }

    fn set(&mut self, c: Coord, v: char) {
        if !self.in_bounds(c.x, c.y) {
            println!("set out of bounds");
        }

        self.data[c.y as usize][c.x as usize] = v;
    }

    #[allow(dead_code)]
    fn pretty_map(&self) {
        let cloned = self.data.clone();
        let out = cloned.into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        println!("{out}");
    }

    // lazy
    fn find_start(data: &Vec<Vec<char>>) -> Coord {
        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c == &'S' {
                    return Coord { x: x as i32, y: y as i32 };
                }
            }
        }

        Coord { x: -1, y: -1 }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.xmax && y < self.ymax
    }

    fn get(&self, c: Coord) -> Option<char> {
        if !self.in_bounds(c.x, c.y) {
            None
        } else {
            Some(self.data[c.y as usize][c.x as usize])
        }
    }

    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.

    // Returns a vec of the tiles that the animal can step to.
    fn get_adjoining_steps(&self, animal: Coord) -> Vec<Coord> {
        let mut nexts = vec![];

        // north - OK for all tiles connected to the south
        if let Some(new) = self.get(animal.north()) {
            if new == '|' || new == '7' || new == 'F' {
                nexts.push(animal.north());
            }
        }

        // south - OK for all tiles connected to the north
        if let Some(new_tile) = self.get(animal.south()) {
            if new_tile == '|' || new_tile == 'L' || new_tile == 'J' {
                nexts.push(animal.south());
            }
        }

        // east - OK for all tiles connected to the west
        if let Some(new_tile) = self.get(animal.east()) {
            if new_tile == '-' || new_tile == 'J' || new_tile == '7' {
                nexts.push(animal.east());
            }
        }
        // west - OK for all tiles connected to the east
        if let Some(new_tile) = self.get(animal.west()) {
            if new_tile == '-' || new_tile == 'L' || new_tile == 'F' {
                nexts.push(animal.west());
            }
        }

        nexts
    }
    /// `animal` is where the animal is (was).
    /// `tile` is where we are checking (is).
    /// if animal can pass through tile, we return the coord of the next location.
    /// otherwise, return None.
    fn step(&self, prev: Coord, tile: Coord) -> Option<Coord> {
        let cur_tile = self.get(tile)?;

        if cur_tile == '.' {
            return None;
        }

        // left-right as same y
        if prev.y == tile.y {
            // animal coming from left (west): - moves east, J moves north, 7 moves south.
            if prev.x == tile.x - 1 {
                return match cur_tile {
                    '-' => Some(tile.east()),
                    'J' => Some(tile.north()),
                    '7' => Some(tile.south()),
                    _ => None,
                }
            }

            // animal coming from right (east): - moves west, L moves north, F moves south.
            if prev.x == tile.x + 1 {
                return match cur_tile {
                    // west
                    '-' => Some(tile.west()),
                    // north
                    'L' => Some(tile.north()),
                    // south
                    'F' => Some(tile.south()),
                    _ => None,
                }
            }

            println!("step() error: same y but more than one tile apart");
            return None;
        }

        if prev.x == tile.x {
            // animal coming from above (north): | moves south, L moves east, J moves west.
            if prev.y == tile.y - 1 {
                return match cur_tile {
                    // south
                    '|' => Some(tile.south()),
                    // east
                    'L' => Some(tile.east()),
                    // west
                    'J' => Some(tile.west()),
                    _ => None,
                }
            }

            // animal coming from below (south): | moves north, 7 moves west, F moves east.
            if prev.y == tile.y + 1 {
                return match cur_tile {
                    // north
                    '|' => Some(tile.north()),
                    // west
                    '7' => Some(tile.west()),
                    // east
                    'F' => Some(tile.east()),
                    _ => None,
                }
            }

            println!("step() error: same x but more than one tile apart");
            return None;
        }

        println!("warning: invalid step() check");
        None
    }
}

fn a(map: &Map) -> u64 {
    let mut steps_map = Map::empty(map.xmax, map.ymax);
    steps_map.set(map.start, '0');

    let mut cur_char = '0';

    // -- first step --
    let mut currents = map.get_adjoining_steps(map.start);
    let mut prevs = currents.iter().map(|_| map.start).collect::<Vec<_>>();

    // set map
    cur_char = incr_char(cur_char);
    currents.iter().for_each(|c| steps_map.set(*c, cur_char));

    let mut i = 1;
    loop {
        i += 1;
        let nexts = currents.iter().zip(prevs.iter())
            .map(|(cur, prev)| map.step(*prev, *cur).unwrap())
            .collect::<Vec<_>>();
        prevs = currents;
        currents = nexts;

        // update and print map
        cur_char = incr_char(cur_char);
        currents.iter().for_each(|c| steps_map.set(*c, cur_char));
        println!("\n======================= {cur_char}");
        steps_map.pretty_map();

        // check if all the same
        if currents.iter().all(|x| *x == currents[0]) {
            return i;
        }
    }
}

fn incr_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}

fn b(map: &Map) -> u64 {
    let mut steps_map = Map::empty(map.xmax, map.ymax);
    steps_map.set(map.start, '0');

    let mut set = HashSet::new();
    set.insert(map.start);

    // -- first step --
    let mut currents = map.get_adjoining_steps(map.start);
    let mut prevs = currents.iter().map(|_| map.start).collect::<Vec<_>>();

    // set map
    currents.iter().for_each(|c| steps_map.set(*c, 'X'));
    currents.iter().for_each(|c| { set.insert(*c); });

    loop {
        // process step and increment prev/currents
        let nexts = currents.iter().zip(prevs.iter())
            .map(|(cur, prev)| map.step(*prev, *cur).unwrap())
            .collect::<Vec<_>>();
        prevs = currents;
        currents = nexts;

        // update map and set
        currents.iter().for_each(|c| { set.insert(*c); });
        currents.iter().for_each(|c| steps_map.set(*c, 'X'));

        // check if all the same
        if currents.iter().all(|x| *x == currents[0]) {
            break;
        }
    }

    steps_map.pretty_map();

    let loop_xmin = set.iter().map(|c| c.x).min().unwrap();
    let loop_ymin = set.iter().map(|c| c.y).min().unwrap();
    let loop_xmax = set.iter().map(|c| c.x).max().unwrap();
    let loop_ymax = set.iter().map(|c| c.y).max().unwrap();
    println!("{:?}", set.iter().map(|x| x.pretty()).collect::<Vec<_>>().join(", "));

    let mut i = 0;
    for y in 0..map.ymax {
        for x in 0..map.xmax {
            if x < loop_xmin || x > loop_xmax|| y < loop_ymin || y > loop_ymax {
                continue;
            }

            if set.contains(&Coord { x, y }) {
                continue;
            }
            println!("({x},{y}) {:?}", steps_map.get(Coord{ x, y }));

            i += 1;
        }
    }

    i
}

fn main() {
    Puzzle {
        name: "10",
        //parts: vec![a, b],
        parts: vec![b],
        delimiter: '\n',
        preprocess: |text| {
            Map::new(text.into_iter().map(|line| line.chars().collect()).collect())
        },
    }.solve();
}
