mod cell_patterns;
mod universe;
mod utils;

use universe::{Materials, Universe};
use utils::{Position, SizeFloat, SizeInt};

use bevy::{prelude::*, render::camera::Camera};
use std::time::Duration;

/// Configuration for universe generation
struct GenerationConfig {
    /// The initial size of the universe
    initial_size: SizeInt,
    /// How likely it is for a cell to be alive when generating the universe, a number between 0.0 - 1.0
    life_chance: f32,
}
impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            initial_size: SizeInt::new(32, 32),
            life_chance: 0.4,
        }
    }
}

struct SimulationConfig {
    /// Extra padding added to the universe's bounds
    bound_padding: i32,
    /// How often the universe updates
    tick_interval: Timer,
    paused: bool,
    /// How many neighbors a cell can live with
    allowed_neighbors: Vec<u8>,
    /// How many neighbors are required for a dead cell to become a live cell, as if by reproduction
    allowed_neighbors_for_birth: Vec<u8>,
    generation: GenerationConfig,
}
impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            bound_padding: 5,
            tick_interval: Timer::new(Duration::from_secs_f32(0.5), true),
            paused: false,
            allowed_neighbors: vec![2, 3],
            allowed_neighbors_for_birth: vec![3],
            generation: GenerationConfig::default(),
        }
    }
}

struct CursorPosition {
    x: f32,
    y: f32,
}

struct DrawnPositions(Vec<Position>);

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
        sim_config.generation.initial_size,
        sim_config.generation.life_chance,
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
        if sim_config.tick_interval.tick(time.delta()).just_finished() && !sim_config.paused {
            universe.tick(
                &mut commands,
                &sim_config.allowed_neighbors,
                &sim_config.allowed_neighbors_for_birth,
            );
        }
    }
}

// TODO: Fix drawing, the position is a bit wrong
fn draw_cells(
    mut commands: Commands,
    windows: Res<Windows>,
    mut sim_config: ResMut<SimulationConfig>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut drawn_positions: ResMut<DrawnPositions>,
    mut universes: Query<&mut Universe>,
) {
    if let Ok(mut universe) = universes.single_mut() {
        if mouse_button_input.pressed(MouseButton::Left) {
            sim_config.paused = true;
            let window = windows.get_primary().unwrap();
            let game_size = window.width().min(window.height());
            let bounds = universe.bounds().with_padding(sim_config.bound_padding);
            let universe_size = bounds.size();
            let cursor_pos = Position::new(
                (cursor_position.x / (game_size / universe_size.width as f32)) as i32,
                (cursor_position.y / (game_size / universe_size.height as f32)) as i32,
            );
            if !drawn_positions.0.contains(&cursor_pos) {
                universe.toggle_cells_at(
                    &mut commands,
                    vec![Position::new(cursor_pos.x, cursor_pos.y)],
                );
                drawn_positions.0.push(cursor_pos);
            }
        } else if mouse_button_input.just_released(MouseButton::Left) {
            sim_config.paused = false;
            drawn_positions.0.clear();
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
        let window = windows.get_primary().unwrap();
        let game_size = window.width().min(window.height());
        let bounds = universe.bounds().with_padding(sim_config.bound_padding);
        let universe_size = bounds.size();
        for (pos, mut transform) in query.iter_mut() {
            transform.translation = Vec3::new(
                convert(
                    (pos.x - bounds.left) as f32,
                    game_size,
                    universe_size.width as f32,
                ),
                convert(
                    (pos.y - bounds.bottom) as f32,
                    game_size,
                    universe_size.height as f32,
                ),
                0.0,
            );
        }
    }
}

// TODO: Fix scaling, sprites get stretched in some cases when the bounds don't form a square
fn size_scaling(
    windows: Res<Windows>,
    sim_config: ResMut<SimulationConfig>,
    universes: Query<&Universe>,
    mut query: Query<(&SizeFloat, &mut Sprite)>,
) {
    if let Ok(universe) = universes.single() {
        let window = windows.get_primary().unwrap();
        let game_size = window.width().min(window.height());
        let bounds = universe.bounds().with_padding(sim_config.bound_padding);
        let universe_size = bounds.size();
        for (sprite_size, mut sprite) in query.iter_mut() {
            sprite.size = Vec2::new(
                sprite_size.width / universe_size.width as f32 * game_size,
                sprite_size.height / universe_size.height as f32 * game_size,
            );
        }
    }
}

/// Gets the cursor position in world coordinates
fn cursor_position(
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let pos = pos - size / 2.0;

        let camera_transform = camera.single().unwrap();

        let cursor_pos_world = camera_transform.compute_matrix() * pos.extend(0.0).extend(1.0);
        cursor_position.x = cursor_pos_world.x;
        cursor_position.y = cursor_pos_world.y;
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
            allowed_neighbors: vec![2, 3],
            allowed_neighbors_for_birth: vec![3],
            ..Default::default()
        })
        .insert_resource(CursorPosition { x: 0.0, y: 0.0 })
        .insert_resource(DrawnPositions(vec![]))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .add_system(universe.system())
        .add_system(cursor_position.system())
        .add_system(draw_cells.system())
        .run();
}
