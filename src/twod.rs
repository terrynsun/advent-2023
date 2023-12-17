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

#[derive(Debug, Clone)]
pub struct Map {
    pub data: Vec<Vec<char>>,

    // These should be usize but it's annoying to deal with underflows.
    pub xmax: i32,
    pub ymax: i32,
}

impl Map {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        let ymax = data.len() as i32;
        let xmax = data[0].len() as i32;

        Self { data, ymax, xmax }
    }

    pub fn from_strings(data: Vec<String>) -> Self {
        Map::new(data.into_iter().map(|line| line.chars().collect()).collect())
    }

    pub fn empty(xmax: i32, ymax: i32) -> Self {
        let data = (0..ymax)
            .map(|_| (0..xmax).map(|_| '.').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self::new(data)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.xmax && y < self.ymax
    }

    pub fn get(&self, c: Coord) -> Option<char> {
        if !self.in_bounds(c.x, c.y) {
            None
        } else {
            Some(self.data[c.y as usize][c.x as usize])
        }
    }

    pub fn set(&mut self, c: Coord, v: char) {
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
