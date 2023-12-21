use std::collections::HashSet;

use advent_2023::puzzle::Puzzle;
use advent_2023::twod::{Map, char_map_from_strings};

fn a(map: &Map<char>) -> usize {
    let mut tracker_map = map.clone();

    let mut locs = HashSet::new();
    locs.insert(map.find(&'S'));

    for _ in 0..64 {
        let mut next_steps = HashSet::new();

        for location in locs {
            tracker_map.set(location, '.');
            let possible_next_steps = [
                location.north(),
                location.east(),
                location.south(),
                location.west(),
            ];

            for try_step in possible_next_steps {
                if let Some(ground) = map.get(try_step) {
                    if ground != '#' {
                        next_steps.insert(try_step);
                        tracker_map.set(try_step, 'O');
                    }
                }
            }
        }

        locs = next_steps;
    }

    locs.len()
}

fn b(map: &Map<char>) -> usize {
    let mut tracker_map = map.clone();

    let mut locs = HashSet::new();
    locs.insert(map.find(&'S'));

    for i in 0..26501365 {
        if i % 5 == 0 {
            println!("\n{i}");
            println!("{tracker_map}");
            println!("{:?}", locs.len());
        }
        let mut next_steps = HashSet::new();

        for location in locs {
            tracker_map.set(location, '.');
            let possible_next_steps = [
                location.north(),
                location.east(),
                location.south(),
                location.west(),
            ];

            for try_step in possible_next_steps {
                let wrapped_step = map.wrap_coord(try_step);

                let ground = map.get(wrapped_step).unwrap();
                if ground != '#' {
                    next_steps.insert(wrapped_step);
                    tracker_map.set(wrapped_step, 'O');
                }
            }
        }

        locs = next_steps;
    }

    locs.len()
}

fn main() {
    Puzzle {
        name: "21",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: char_map_from_strings,
    }
    .solve();
}
