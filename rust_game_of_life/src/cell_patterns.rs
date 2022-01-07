use crate::utils::Position;

pub struct CellPattern {
    pub cells: Vec<Position>,
}
impl CellPattern {
    pub fn new(cells: Vec<Position>) -> CellPattern {
        CellPattern { cells }
    }
    pub fn glider() -> CellPattern {
        CellPattern::new(vec![
            Position::new(0, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 0),
            Position::new(2, 1),
        ])
    }
}
