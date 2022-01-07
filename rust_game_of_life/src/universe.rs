// TODO: Decouple from game engine

use std::{collections::HashMap, fmt, i32::MAX};

use bevy::prelude::*;
use rand::random;

use crate::utils::{Position, SizeFloat, SizeInt};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub entity: Entity,
}
impl Cell {
    fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}
impl Bounds {
    pub fn with_padding(&self, padding: i32) -> Self {
        Self {
            top: self.top + padding,
            right: self.right + padding,
            bottom: self.bottom - padding,
            left: self.left - padding,
        }
    }
    pub fn size(&self) -> SizeInt {
        SizeInt::new(
            (self.left - self.right).abs(),
            (self.top - self.bottom).abs(),
        )
    }
}

#[derive(Clone, Default)]
pub struct Materials {
    pub cell_alive: Handle<ColorMaterial>,
}

/// A `HashMap` containing the positions and entities of all living cells
pub type Cells = HashMap<Position, Cell>;

#[derive(Clone, Default)]
pub struct Universe {
    pub cells: Cells,
    pub materials: Materials,
}
impl Universe {
    pub fn new(cells: Cells, materials: Materials) -> Self {
        Self { cells, materials }
    }
    pub fn bounds(&self) -> Bounds {
        let mut bounds = Bounds {
            top: -MAX,
            bottom: MAX,
            left: MAX,
            right: -MAX,
        };
        for (pos, _) in &self.cells {
            if pos.y > bounds.top {
                bounds.top = pos.y;
            }
            if pos.y < bounds.bottom {
                bounds.bottom = pos.y;
            }
            if pos.x < bounds.left {
                bounds.left = pos.x;
            }
            if pos.x > bounds.right {
                bounds.right = pos.x;
            }
        }
        bounds
    }
    pub fn toggle_cells_at(&mut self, commands: &mut Commands, positions: Vec<Position>) {
        for pos in positions.iter().cloned() {
            let cell = &mut self.cells.get(&pos);
            match cell {
                Some(data) => {
                    self.despawn_cell_entity(commands, data.entity);
                    self.cells.remove(&pos);
                }
                None => {
                    self.cells
                        .insert(pos, Cell::new(self.spawn_cell_entity(commands, pos)));
                }
            }
        }
    }
    fn spawn_cell_entity(&self, commands: &mut Commands, pos: Position) -> Entity {
        let entity = commands.spawn().id();
        commands
            .entity(entity)
            .insert(Cell::new(entity))
            .insert_bundle(SpriteBundle {
                material: self.materials.cell_alive.clone(),
                ..Default::default()
            })
            .insert(pos)
            .insert(SizeFloat::new(1.0, 1.0));
        entity
    }
    fn despawn_cell_entity(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).despawn_recursive();
    }
    pub fn generate(
        commands: &mut Commands,
        materials: Materials,
        size: SizeInt,
        life_chance: f32,
    ) -> Self {
        let mut cells: Cells = HashMap::new();
        let half_size = SizeInt::new(
            (size.width as f32 / 2.0) as i32,
            (size.height as f32 / 2.0) as i32,
        );
        for y in -half_size.height..half_size.height {
            for x in -half_size.width..half_size.width {
                let lives = random::<f32>() < life_chance;
                if lives {
                    cells.insert(Position::new(x, y), Cell::new(commands.spawn().id()));
                }
            }
        }
        Self::new(cells, materials)
    }
    pub fn live_neighbor_count(&self, pos: Position) -> u8 {
        let mut count = 0;
        for neighbor_pos in pos.neighbors() {
            if self.cells.get(&neighbor_pos).is_some() {
                count += 1;
            }
        }
        count
    }
    /// Plays one frame of the simulation.
    ///
    /// ## Arguments
    ///
    /// - `allowed_neighbors` - How many neighbors a cell can live with
    /// - `allowed_neighbors_for_birth` - How many neighbors are required for a dead cell to become a live cell, as if by reproduction
    pub fn tick(
        &mut self,
        commands: &mut Commands,
        allowed_neighbors: &Vec<u8>,
        allowed_neighbors_for_birth: &Vec<u8>,
    ) {
        let mut next: Cells = self.cells.clone();
        let mut visited: Vec<Position> = vec![];
        for (pos, cell) in self.cells.iter() {
            if visited.contains(&pos) {
                continue;
            }

            // Die if too many/not enough neighbors.
            let live_neighbors = self.live_neighbor_count(pos.to_owned());
            let dies = !allowed_neighbors.contains(&live_neighbors);
            if dies {
                self.despawn_cell_entity(commands, cell.entity);
                next.remove(&pos);
            }

            // Loop through dead neighbors.
            // Neighbors become alive if they have the right amount of neighbors.
            for neighbor_pos in pos.neighbors() {
                if visited.contains(&neighbor_pos) || self.cells.get(&neighbor_pos).is_some() {
                    continue;
                }
                let neighbor_cell = self.cells.get(&neighbor_pos);
                let neighbor_live_neighbors = self.live_neighbor_count(neighbor_pos);
                let is_born = neighbor_cell.is_none()
                    && allowed_neighbors_for_birth.contains(&neighbor_live_neighbors);

                if is_born {
                    // Neighbor is born, insert into next generation and spawn entity
                    next.insert(
                        neighbor_pos,
                        Cell::new(self.spawn_cell_entity(commands, neighbor_pos)),
                    );
                }
                visited.push(neighbor_pos);
            }
            visited.push(pos.to_owned());
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bounds = self.bounds();
        info!("{:?}", bounds);
        for y in (bounds.bottom..bounds.top + 1).rev() {
            write!(f, "\n")?;
            for x in bounds.left..bounds.right + 1 {
                let cell = self.cells.get(&Position::new(x, y));
                let symbol = if cell.is_some() { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
        }
        Ok(())
    }
}
