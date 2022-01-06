#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SizeInt {
    pub width: u32,
    pub height: u32,
}
impl SizeInt {
    pub fn new(width: u32, height: u32) -> Self {
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
