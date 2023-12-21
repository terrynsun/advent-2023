use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;
use advent_2023::twod::Coord;
use advent_2023::twod::Map as InnerMap;

#[derive(Debug)]
struct Map {
    pub inner: InnerMap<char>,

    pub start: Coord,
}

impl Map {
    fn new(data: Vec<Vec<char>>) -> Self {
        let inner = InnerMap::new(data);
        let start = inner.find(&'S');

        Self { inner, start }
    }

    fn empty(xmax: i32, ymax: i32) -> Self {
        Self {
            inner: InnerMap::empty(xmax, ymax, '.'),
            start: Coord::new(0, 0),
        }
    }

    fn set(&mut self, c: Coord, v: char) {
        self.inner.set(c, v);
    }

    fn get(&self, c: Coord) -> Option<char> {
        self.inner.get(c)
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
    let mut steps_map = Map::empty(map.inner.xmax, map.inner.ymax);
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
    let mut steps_map = Map::empty(map.inner.xmax, map.inner.ymax);
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

    println!("{}", steps_map.inner);

    let mut inside = 0;
    for (y, line) in steps_map.inner.data.clone().iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'X' {
                continue;
            }

            let rem = map.inner.data[y][x..].iter().filter(|&&x| x == 'X').count();
            if rem % 2 == 1 {
                inside += 1;
                steps_map.set(Coord { x: x as i32, y: y as i32 }, 'I');
            }
        }
    }

    println!("{}", steps_map.inner);

    inside
}

fn main() {
    Puzzle {
        name: "10",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            Map::new(text.into_iter().map(|line| line.chars().collect()).collect())
        },
    }.solve();
}
