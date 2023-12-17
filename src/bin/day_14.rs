use advent_2023::puzzle::Puzzle;
use advent_2023::twod::Coord;

#[derive(Debug, Clone)]
struct Map {
    pub data: Vec<Vec<char>>,

    // These should be usize but it's annoying to deal with underflows.
    pub xmax: i32,
    pub ymax: i32,
}

impl Map {
    fn new(data: Vec<Vec<char>>) -> Self {
        let ymax = data.len() as i32;
        let xmax = data[0].len() as i32;

        Self { data, ymax, xmax }
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

    fn set(&mut self, c: Coord, v: char) {
        if !self.in_bounds(c.x, c.y) {
            println!("set out of bounds");
        }

        self.data[c.y as usize][c.x as usize] = v;
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.data.iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{out}")
    }
}

// ----- day 14 ----- //

impl Map {
    fn get_first_northern_rock(&self, x: i32, y: i32) -> Coord {
        let mut final_coord = Coord { x, y };
        // this could be a weird fold or a search
        for i in (0..y).rev() {
            let target_obj = self.get(Coord { x, y: i }).unwrap();
            match target_obj  {
                // ok to move here
                '.' => final_coord = Coord { x , y: i },
                // roadblock, return as far as we've slid
                'O' | '#' => {
                    return final_coord;
                },
                _ => panic!("invalid symbol"),
            }
        }

        final_coord
    }
}

fn a(map: &Map) -> usize {
    let mut map = map.clone();
    println!("{map}");
    for (y, line) in map.data.clone().iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'O' {
                let new_coord = map.get_first_northern_rock(x as i32, y as i32);
                map.set(Coord::from_usize(x, y), '.');
                map.set(new_coord, 'O');
            }
        }
    }

    println!("\n{map}");
    map.data.iter().enumerate()
        .map(|(y, line)| { line.iter().filter(|&&c| c == 'O').count() * (map.ymax as usize - y)})
        .sum::<usize>()
}

fn b(_data: &Map) -> usize {
    0
}

fn main() {
    Puzzle {
        name: "14",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            Map::new(text.into_iter().map(|line| line.chars().collect()).collect())
        },
    }.solve();
}
