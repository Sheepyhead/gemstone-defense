use bevy::prelude::*;
use leafwing_input_manager::{action_state::ActionState, plugin::InputManagerPlugin, Actionlike};

use crate::{player::MapCursorPosition, GameState};

pub const MAP_HEIGHT: u32 = 34 * 4;
pub const MAP_WIDTH: u32 = 27 * 4;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MapActions>::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, dig_at_cursor.run_if(in_state(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes
                .add(
                    Plane3d::default()
                        .mesh()
                        .size(MAP_WIDTH as f32, MAP_HEIGHT as f32),
                )
                .into(),
            material: materials.add(Color::GREEN),
            transform: Transform::from_translation(Vec3::new(
                (MAP_WIDTH / 2) as f32,
                0.,
                (MAP_HEIGHT / 2) as f32,
            )),
            ..default()
        },
        Ground,
    ));
}

#[derive(Component)]
pub struct Ground;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MapActions {
    DigAtCursor,
}

fn dig_at_cursor(
    map_cursor_position: Res<MapCursorPosition>,
    action_state: Query<&ActionState<MapActions>>,
) {
    for action_state in &action_state {
        if action_state.just_pressed(&MapActions::DigAtCursor) {
            let (x, y) = map_cursor_position.to_coords();
            info!("Digging at cursor {},{}", x, y);
        }
    }
}
