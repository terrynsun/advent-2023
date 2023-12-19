#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;
use advent_2023::twod::{Map, Coord};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Beam {
    coord: Coord,
    dir: Direction,

    // maxes out at 3
    steps_in_this_dir: u8,
}

impl Beam {
    fn get_possible_next_moves(&self, map: &Map<u32>) -> Vec<Beam> {
        // todo
        vec![]
    }

    fn step_forward(&mut self) {
        self.coord = match self.dir {
            Direction::North => self.coord.north(),
            Direction::East => self.coord.east(),
            Direction::South => self.coord.south(),
            Direction::West => self.coord.west(),
        }
    }
}

fn a(map: &Map<u32>) -> u64 {
    let mut beam_history = HashSet::new();
    let mut beams = vec![
        Beam {
            coord: Coord::new(0, 0),
            dir: Direction::East,
            steps_in_this_dir: 0,
        },
        Beam {
            coord: Coord::new(0, 0),
            dir: Direction::South,
            steps_in_this_dir: 0,
        },
    ];

    loop {
        beams = beams.iter()
            .flat_map(|b| b.get_possible_next_moves(map))
            .collect();

        // everything below shared with day16
        beams.iter_mut()
            .for_each(|b| b.step_forward());

        beams = beams
            .into_iter()
            // discard beams that have left the map
            .filter(|b| map.in_bounds(b.coord))
            // discard beams that we have seen before
            .filter(|b| !beam_history.contains(b))
            .collect();

        beams.iter()
            .for_each(|b| { beam_history.insert(*b); });

        if beams.is_empty() {
            break;
        }
    }

    0
}

fn b(map: &Map<u32>) -> u64 {
    let mut min_map = Map::empty(map.xmax, map.ymax, 0);

    // target is lower right corner
    let target = Coord::new(map.xmax-1, map.ymax-1);

    // the minimum cost to get into the target is the target value
    min_map.set(target, map.get(target).unwrap());

    // todo...
    0
}

fn main() {
    Puzzle {
        name: "17",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            Map::new(
                text.into_iter()
                    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                    .collect(),
            )
        },
    }
    .solve();
}
