mod cell_patterns;
mod universe;
mod utils;

use universe::{Materials, Universe};
use utils::{Position, SizeFloat, SizeInt};

use bevy::prelude::*;
use std::time::Duration;

struct SimulationConfig {
    bound_padding: i32,
    tick_interval: Timer,
    /// How many neighbors a cell can live with
    allowed_neighbors: Vec<u8>,
    /// How many neighbors are required for a dead cell to become a live cell, as if by reproduction
    allowed_neighbors_for_birth: Vec<u8>,
    /// Used when generating cells
    life_chance: f32,
}
impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            bound_padding: 5,
            tick_interval: Timer::new(Duration::from_secs_f32(0.5), true),
            allowed_neighbors: vec![2, 3],
            allowed_neighbors_for_birth: vec![3],
            life_chance: 0.4,
        }
    }
}

fn setup(
    mut commands: Commands,
    sim_config: Res<SimulationConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let materials = Materials {
        cell_alive: materials.add(Color::rgb(0.4, 1.0, 0.6).into()),
    };
    commands.insert_resource(materials.clone());
    setup_universe(&mut commands, sim_config, materials)
}

fn setup_universe(
    commands: &mut Commands,
    sim_config: Res<SimulationConfig>,
    materials: Materials,
) {
    let universe = Universe::generate(
        commands,
        materials,
        SizeInt::new(32, 32),
        sim_config.life_chance,
    );
    commands.spawn().insert(universe);
}

fn universe(
    mut commands: Commands,
    time: Res<Time>,
    mut sim_config: ResMut<SimulationConfig>,
    mut query: Query<&mut Universe>,
) {
    if let Ok(mut universe) = query.single_mut() {
        if sim_config.tick_interval.tick(time.delta()).just_finished() {
            universe.tick(
                &mut commands,
                &sim_config.allowed_neighbors,
                &sim_config.allowed_neighbors_for_birth,
            );
        }
    }
}

fn position_translation(
    windows: Res<Windows>,
    sim_config: ResMut<SimulationConfig>,
    universes: Query<&Universe>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    if let Ok(universe) = universes.single() {
        fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
            let tile_size = bound_window / bound_game;
            pos * tile_size - bound_window / 2.0
        }
        let bounds = universe.bounds().with_padding(sim_config.bound_padding);
        let window = windows.get_primary().unwrap();
        let universe_size = SizeInt::new(
            (bounds.left - bounds.right).abs(),
            (bounds.top - bounds.bottom).abs(),
        );
        for (pos, mut transform) in query.iter_mut() {
            transform.translation = Vec3::new(
                convert(
                    (pos.x - bounds.left) as f32,
                    window.width(),
                    universe_size.width as f32,
                ),
                convert(
                    (pos.y - bounds.bottom) as f32,
                    window.height(),
                    universe_size.height as f32,
                ),
                0.0,
            );
        }
    }
}

fn size_scaling(
    windows: Res<Windows>,
    sim_config: ResMut<SimulationConfig>,
    universes: Query<&Universe>,
    mut query: Query<(&SizeFloat, &mut Sprite)>,
) {
    if let Ok(universe) = universes.single() {
        let window = windows.get_primary().unwrap();
        let bounds = universe.bounds().with_padding(sim_config.bound_padding);
        let universe_size = SizeInt::new(
            (bounds.left - bounds.right).abs(),
            (bounds.top - bounds.bottom).abs(),
        );
        for (sprite_size, mut sprite) in query.iter_mut() {
            sprite.size = Vec2::new(
                sprite_size.width / universe_size.width as f32 * window.width(),
                sprite_size.height / universe_size.height as f32 * window.height(),
            );
        }
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("Bevy Conway's game of life"),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(SimulationConfig {
            tick_interval: Timer::new(Duration::from_secs_f32(0.1), true),
            allowed_neighbors_for_birth: vec![3],
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .add_system(universe.system())
        .run();
}
