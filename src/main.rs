mod universe;
mod utils;

use universe::{Cell, CellState, Universe};
use utils::{Position, SizeFloat, SizeInt};

use bevy::prelude::*;
use std::time::Duration;

struct SimulationConfig {
    universe_size: SizeInt,
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
            universe_size: SizeInt::new(32, 32),
            tick_interval: Timer::new(Duration::from_secs_f32(0.5), true),
            allowed_neighbors: vec![2, 3],
            allowed_neighbors_for_birth: vec![3],
            life_chance: 0.4,
        }
    }
}

#[derive(Clone, Default)]
struct Materials {
    cell_dead: Handle<ColorMaterial>,
    cell_born: Handle<ColorMaterial>,
    cell_alive: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    sim_config: Res<SimulationConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let materials = Materials {
        cell_dead: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        cell_born: materials.add(Color::rgb(0.1, 0.3, 0.2).into()),
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
    let mut universe = Universe::generate(sim_config.universe_size, sim_config.life_chance);
    for y in 0..universe.size.height {
        for x in 0..universe.size.width {
            let cell = &mut universe.cells[y as usize][x as usize];
            let entity = commands
                .spawn()
                .insert(Cell)
                .insert_bundle(SpriteBundle {
                    material: match cell.state {
                        CellState::Dead => materials.cell_dead.clone(),
                        CellState::Alive => materials.cell_alive.clone(),
                    },
                    ..Default::default()
                })
                .insert(Position::new(x, y))
                .insert(SizeFloat::new(1.0, 1.0))
                .id();
            cell.entity = Some(entity);
        }
    }
    commands.spawn().insert(universe);
}

fn universe(
    time: Res<Time>,
    mut sim_config: ResMut<SimulationConfig>,
    materials: Res<Materials>,
    mut query: Query<&mut Universe>,
    mut sprite_materials: Query<&mut Handle<ColorMaterial>, With<Cell>>,
) {
    if sim_config.tick_interval.tick(time.delta()).just_finished() {
        for mut universe in query.iter_mut() {
            let prev_universe = universe.clone();
            universe.tick(
                &sim_config.allowed_neighbors,
                &sim_config.allowed_neighbors_for_birth,
            );
            for y in 0..universe.size.height as usize {
                for x in 0..universe.size.width as usize {
                    let cell = universe.cells[y][x];
                    let prev_cell_state = prev_universe.cells[y][x].state;
                    let just_born =
                        prev_cell_state == CellState::Dead && cell.state == CellState::Alive;
                    let not_changed = cell.state != prev_cell_state && !just_born;
                    if !not_changed || just_born {
                        let material = if cell.state == CellState::Dead {
                            materials.cell_dead.clone()
                        } else if just_born {
                            materials.cell_born.clone()
                        } else {
                            materials.cell_alive.clone()
                        };
                        match cell.entity {
                            Some(ent) => sprite_materials
                                .get_mut(ent)
                                .unwrap()
                                .set(Box::new(material))
                                .unwrap(),
                            None => (),
                        };
                    }
                }
            }
        }
    }
}

fn position_translation(
    windows: Res<Windows>,
    sim_config: Res<SimulationConfig>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width() as f32,
                sim_config.universe_size.width as f32,
            ),
            convert(
                pos.y as f32,
                window.height() as f32,
                sim_config.universe_size.height as f32,
            ),
            0.0,
        );
    }
}

fn size_scaling(
    windows: Res<Windows>,
    sim_config: Res<SimulationConfig>,
    mut query: Query<(&SizeFloat, &mut Sprite)>,
) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in query.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / sim_config.universe_size.width as f32 * window.width(),
            sprite_size.height / sim_config.universe_size.height as f32 * window.height(),
        );
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
            universe_size: SizeInt::new(128, 128),
            tick_interval: Timer::new(Duration::from_secs_f32(0.05), true),
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
