use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn step(&self, dir: Direction) -> Coord {
        match dir {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }

    pub fn north(&self) -> Coord {
        Coord { x: self.x, y: self.y-1 }
    }

    pub fn south(&self) -> Coord {
        Coord { x: self.x, y: self.y+1 }
    }

    pub fn west(&self) -> Coord {
        Coord { x: self.x-1, y: self.y }
    }

    pub fn east(&self) -> Coord {
        Coord { x: self.x+1, y: self.y }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Map<T> {
    pub data: Vec<Vec<T>>,

    // These should be usize but it's annoying to deal with underflows.
    pub xmax: i32,
    pub ymax: i32,
}

pub fn char_map_from_strings(data: Vec<String>) -> Map<char> {
    Map::new(data.into_iter().map(|line| line.chars().collect()).collect())
}

impl<T> Map<T>
where
    T: Copy
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let ymax = data.len() as i32;
        let xmax = data[0].len() as i32;

        Self { data, ymax, xmax }
    }

    pub fn empty(xmax: i32, ymax: i32, val: T) -> Self {
        let data = (0..ymax)
            .map(|_| (0..xmax).map(|_| val).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self::new(data)
    }

    pub fn in_bounds(&self, c: Coord) -> bool {
        c.x >= 0 && c.y >= 0 && c.x < self.xmax && c.y < self.ymax
    }

    pub fn get(&self, c: Coord) -> Option<T> {
        if !self.in_bounds(c) {
            None
        } else {
            Some(self.data[c.y as usize][c.x as usize])
        }
    }

    pub fn set(&mut self, c: Coord, v: T) {
        if !self.in_bounds(c) {
            println!("set out of bounds");
        }

        self.data[c.y as usize][c.x as usize] = v;
    }
}

impl<T> Display for Map<T>
where
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.data.iter()
            .map(|line|
                line.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("")
            )
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{out}")
    }
}
