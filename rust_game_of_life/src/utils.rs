#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors: Vec<Self> = Vec::new();
        for y in self.y - 1..self.y + 2 {
            for x in self.x - 1..self.x + 2 {
                if !(x == self.x && y == self.y) {
                    neighbors.push(Self::new(x, y));
                }
            }
        }
        neighbors
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SizeInt {
    pub width: i32,
    pub height: i32,
}
impl SizeInt {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct SizeFloat {
    pub width: f32,
    pub height: f32,
}
impl SizeFloat {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
