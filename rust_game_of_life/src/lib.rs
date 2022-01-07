//! [Conway's game of life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) recreated in Rust.
//!
//! **WARNING:** This crate does not properly work on it's own yet. It is currently dependent on the Bevy game engine.
//! I originally created this crate for my Bevy version of the game, so things like entities and components are still in the game's logic. I will soon try to make this it's own independent project with a potential CLI.
//!
//! ## Features
//!
//! - Infinite universe
//! - Randomly generated universe
//! - Custom cell patterns and presets
//! - Simulation configuration for things like:
//!     - Tick speed
//!     - Neighbor count required for a cell to be alive/born
//!     - Initial size of randomly generated universes (padding can be added)
//!     - Chance for cell to be alive when generating the universe

use std::time::Duration;

use utils::SizeInt;

pub mod cell_patterns;
pub mod universe;
pub mod utils;

/// Controls various settings related to the simulation and generation of cells
pub struct SimulationConfig {
    /// Extra padding added to the universe's bounds
    pub bound_padding: i32,
    /// How often the universe updates
    pub tick_speed: Duration,
    pub paused: bool,
    /// How many neighbors a cell can live with
    pub allowed_neighbors: Vec<u8>,
    /// How many neighbors are required for a dead cell to become a live cell, as if by reproduction
    pub allowed_neighbors_for_birth: Vec<u8>,
    pub generation: GenerationConfig,
}
impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            bound_padding: 5,
            tick_speed: Duration::from_secs_f32(0.5),
            paused: false,
            allowed_neighbors: vec![2, 3],
            allowed_neighbors_for_birth: vec![3],
            generation: GenerationConfig::default(),
        }
    }
}

/// Configuration for universe generation
pub struct GenerationConfig {
    /// The initial size of the universe
    pub initial_size: SizeInt,
    /// How likely it is for a cell to be alive when generating the universe, a number between 0.0 - 1.0
    pub life_chance: f32,
}
impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            initial_size: SizeInt::new(32, 32),
            life_chance: 0.4,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
