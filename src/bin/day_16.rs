use advent_2023::puzzle::Puzzle;
use advent_2023::twod::{Coord, Map};

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
struct Beam {
    coord: Coord,
    dir: Direction,
}

impl Beam {
    fn bounce(&self, tile: char) -> Vec<Beam> {
        match tile {
            // If the beam encounters empty space (.), it continues in the same direction.
            '.' => vec![self.clone()],
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
                Direction::North | Direction::South => vec![self.clone()],
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
                Direction::East | Direction::West => vec![self.clone()],
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

fn a(map: &Map) -> usize {
    let mut state = Map::empty(map.xmax, map.ymax);

    let start = Beam {
        coord: Coord::new(0, 0),
        dir: Direction::East,
    };

    let mut beams = vec![start];

    loop {
        beams = beams.iter()
            .map(|b| {
                // mark cur location
                state.set(b.coord, '#');

                // move
                let tile = map.get(b.coord).unwrap();
                b.bounce(tile)
            })
            .flatten()
            .collect();

        beams.iter_mut()
            .for_each(|b| b.step_forward());

        beams.iter()
            .for_each(|b| {
                if map.in_bounds(b.coord) {
                    state.set(b.coord, '*')
                }
            });

        beams = beams
            .into_iter()
            .filter(|b| map.in_bounds(b.coord))
            .collect();

        if beams.is_empty() {
            break;
        }
        let score: usize = state
            .data
            .iter()
            .map(|line| line.iter().filter(|&&c| c != '.').count())
            .sum();
        println!("{score}");
        //println!("{beams:?}");
        //println!("{state}");
    }

    state
        .data
        .iter()
        .map(|line| line.iter().filter(|&&c| c == '#').count())
        .sum()
}

fn b(_data: &Map) -> usize {
    0
}

fn main() {
    Puzzle {
        name: "16",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: Map::from_strings,
    }
    .solve();
}
