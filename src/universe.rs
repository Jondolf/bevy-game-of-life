use std::fmt;

use bevy::prelude::*;
use rand::random;

use crate::utils::{Position, SizeInt};

pub struct Cell;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct CellData {
    pub entity: Option<Entity>,
    pub state: CellState,
}

// TODO: Make infinite (HashMap)
#[derive(Clone, Default)]
pub struct Universe {
    pub size: SizeInt,
    pub cells: Vec<Vec<CellData>>,
}
impl Universe {
    pub fn new(size: SizeInt, cells: Vec<Vec<CellData>>) -> Self {
        Self { size, cells }
    }
    pub fn dead(size: SizeInt) -> Self {
        Self::new(
            size,
            vec![
                vec![
                    CellData {
                        entity: None,
                        state: CellState::Dead
                    };
                    size.width as usize
                ];
                size.height as usize
            ],
        )
    }
    pub fn toggle_cells_at(&mut self, positions: Vec<(u32, u32)>) {
        for (x, y) in positions.iter().cloned() {
            let mut cell = &mut self.cells[y as usize][x as usize];
            cell.state = match cell.state {
                CellState::Dead => CellState::Alive,
                CellState::Alive => CellState::Dead,
            };
        }
    }
    pub fn generate(size: SizeInt, life_chance: f32) -> Self {
        let mut universe = Universe::new(size, vec![vec![]; size.height as usize]);
        for row in universe.cells.iter_mut() {
            for _x in 0..size.width {
                row.push(CellData {
                    entity: None,
                    state: if random::<f32>() < life_chance {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    },
                });
            }
        }
        universe
    }
    pub fn size(&self) -> SizeInt {
        SizeInt::new(self.cells[0].len() as u32, self.cells.len() as u32)
    }
    pub fn live_neighbor_count(&self, pos: Position) -> u8 {
        let mut count = 0;
        let size = self.size();
        for delta_row in [size.height - 1, 0, 1].iter().cloned() {
            for delta_col in [size.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_pos = Position {
                    x: ((pos.x + delta_col) % size.width),
                    y: ((pos.y + delta_row) % size.height),
                };
                count += self.cells[neighbor_pos.y as usize][neighbor_pos.x as usize].state as u8;
            }
        }
        count
    }
    /// Plays one frame of the simulation.
    ///
    /// ## Arguments
    ///
    /// * `allowed_neighbors` - How many neighbors a cell can live with
    /// * `allowed_neighbors_for_birth` - How many neighbors are required for a dead cell to become a live cell, as if by reproduction
    pub fn tick(&mut self, allowed_neighbors: &Vec<u8>, allowed_neighbors_for_birth: &Vec<u8>) {
        let mut next = self.cells.clone();
        let size = self.size();

        for y in 0..size.height {
            for x in 0..size.width {
                let cell = self.cells[y as usize][x as usize];
                let live_neighbors = self.live_neighbor_count(Position::new(x, y));

                let next_cell = CellData {
                    entity: cell.entity,
                    state: match (cell.state, live_neighbors) {
                        (CellState::Alive, x) if !allowed_neighbors.contains(&x) => CellState::Dead,
                        (CellState::Alive, x) if allowed_neighbors.contains(&x) => CellState::Alive,
                        (CellState::Dead, x) if allowed_neighbors_for_birth.contains(&x) => {
                            CellState::Alive
                        }
                        (prev, _) => prev,
                    },
                };

                next[y as usize][x as usize] = next_cell;
            }
        }

        self.cells = next;
    }
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.iter() {
            for &cell in line {
                let symbol = if cell.state == CellState::Dead {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
