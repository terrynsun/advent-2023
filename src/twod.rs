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
