use advent_2023::puzzle::Puzzle;
use advent_2023::util::split_to_u64s;

struct Almanac {
    seeds: Vec<u64>,

    maps: [Map; 7],
}

impl Almanac {
    fn resolve_location(&self, n: u64) -> u64 {
        self.maps.iter()
            .fold(n, |acc, map| {
                map.translate(acc)
            })
    }
}

#[derive(Debug, Default)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn add_range(&mut self, r: Range) {
        self.ranges.push(r);
    }

    fn translate(&self, n: u64) -> u64 {
        // for loops for short circuiting
        for r in self.ranges.iter() {
            if let Some(value) = r.translate(n) {
                return value;
            }
        }

        n
    }
}

#[derive(Debug)]
struct Range {
    src: u64,
    dst: u64,
    length: u64,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let parts = split_to_u64s(s);
        Self {
            src: parts[1],
            dst: parts[0],
            length: parts[2],
        }
    }

    fn translate(&self, n: u64) -> Option<u64> {
        if n >= self.src && n < (self.src + self.length) {
            Some(self.dst + (n - self.src))
        } else {
            None
        }
    }
}

fn a(almanac: &Almanac) -> u64 {
    *almanac.maps.iter()
        .fold(almanac.seeds.clone(), |acc, map| {
            acc.iter().map(|x| map.translate(*x)).collect()
        }).iter()
        .min()
        .unwrap()
}

fn b(almanac: &Almanac) -> u64 {
    let mut min = u64::MAX;
    for seed_range in almanac.seeds.chunks(2).collect::<Vec<_>>() {
        for i in seed_range[0]..seed_range[0]+seed_range[1] {
            min = std::cmp::min(min, almanac.resolve_location(i));
        }
    }

    min
}

fn main() {
    Puzzle {
        name: "5",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            // drop 'seeds:'
            let seeds: Vec<_> = split_to_u64s(&text[0][7..]);

            let mut maps: [Map; 7] = Default::default();

            let mut map_idx = 0;
            for line in text[2..].iter() {
                if line.is_empty() {
                    map_idx += 1;
                    continue;
                }

                if line.chars().next().unwrap().is_ascii_alphabetic() {
                    continue;
                }

                maps[map_idx].add_range(Range::from_str(line));
            }

            Almanac { seeds, maps }
        },
    }
    .solve();
}
