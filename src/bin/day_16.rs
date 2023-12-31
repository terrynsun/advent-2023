use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;
use advent_2023::twod::{Coord, Direction, Map, char_map_from_strings};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Beam {
    coord: Coord,
    dir: Direction,
}

impl Beam {
    fn bounce(&self, tile: char) -> Vec<Beam> {
        match tile {
            // If the beam encounters empty space (.), it continues in the same direction.
            '.' => vec![*self],

            // If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending
            // on the angle of the mirror. For instance, a rightward-moving beam that encounters a
            // / mirror would continue upward in the mirror's column, while a rightward-moving beam
            // that encounters a \ mirror would continue downward from the mirror's column.
            '/' => match self.dir {
                Direction::North => vec![self.turn_cloned(Direction::East)],
                Direction::East => vec![self.turn_cloned(Direction::North)],
                Direction::South => vec![self.turn_cloned(Direction::West)],
                Direction::West => vec![self.turn_cloned(Direction::South)],
            },
            '\\' => match self.dir {
                Direction::North => vec![self.turn_cloned(Direction::West)],
                Direction::East => vec![self.turn_cloned(Direction::South)],
                Direction::South => vec![self.turn_cloned(Direction::East)],
                Direction::West => vec![self.turn_cloned(Direction::North)],
            },

            // If the beam encounters the pointy end of a splitter (| or -), the beam passes
            // through the splitter as if the splitter were empty space. For instance, a
            // rightward-moving beam that encounters a - splitter would continue in the same
            // direction.
            // If the beam encounters the flat side of a splitter (| or -), the beam is split into
            // two beams going in each of the two directions the splitter's pointy ends are
            // pointing. For instance, a rightward-moving beam that encounters a | splitter would
            // split into two beams: one that continues upward from the splitter's column and one
            // that continues downward from the splitter's column.
            '|' => match self.dir {
                // pointy end -> empty space
                Direction::North | Direction::South => vec![*self],
                // flat end -> split
                Direction::East | Direction::West => vec![
                    self.turn_cloned(Direction::North),
                    self.turn_cloned(Direction::South),
                ],
            },
            '-' => match self.dir {
                // flat end -> split
                Direction::North | Direction::South => vec![
                    self.turn_cloned(Direction::East),
                    self.turn_cloned(Direction::West),
                ],
                // pointy end -> empty space
                Direction::East | Direction::West => vec![*self],
            },
            _ => panic!("invalid tile {tile}"),
        }
    }

    fn turn_cloned(&self, new_dir: Direction) -> Beam {
        Beam {
            coord: self.coord,
            dir: new_dir,
        }
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

fn score_from_start(map: &Map<char>, start: Beam) -> usize{
    let mut state = Map::empty(map.xmax, map.ymax, '.');
    let mut beam_history = HashSet::new();
    let mut beams = vec![start];

    loop {
        beams = beams.iter()
            .flat_map(|b| {
                // mark cur location
                state.set(b.coord, '#');

                // move
                let tile = map.get(b.coord).unwrap();
                b.bounce(tile)
            })
            .collect();

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

    state
        .data
        .iter()
        .map(|line| line.iter().filter(|&&c| c == '#').count())
        .sum()
}

fn a(map: &Map<char>) -> usize {
    let start = Beam {
        coord: Coord::new(0, 0),
        dir: Direction::East,
    };
    score_from_start(map, start)
}

fn b(map: &Map<char>) -> usize {
    [
        // top row, going south
        (0..map.xmax)
            .map(|start_x| {
                let start = Beam {
                    coord: Coord::new(start_x, 0),
                    dir: Direction::South,
                };
                score_from_start(map, start)
            })
            .max().unwrap(),

        // bottom row, going north
        (0..map.xmax)
            .map(|start_x| {
                let start = Beam {
                    coord: Coord::new(start_x, map.ymax-1),
                    dir: Direction::North,
                };
                score_from_start(map, start)
            })
            .max().unwrap(),

        // left column, going east
        (0..map.ymax)
            .map(|start_y| {
                let start = Beam {
                    coord: Coord::new(0, start_y),
                    dir: Direction::East,
                };
                score_from_start(map, start)
            })
            .max().unwrap(),

        // right column, going west
        (0..map.ymax)
            .map(|start_y| {
                let start = Beam {
                    coord: Coord::new(map.xmax-1, start_y),
                    dir: Direction::West,
                };
                score_from_start(map, start)
            })
            .max().unwrap(),
    ].into_iter().max().unwrap()
}

fn main() {
    Puzzle {
        name: "16",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: char_map_from_strings,
    }
    .solve();
}
