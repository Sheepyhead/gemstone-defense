use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::GameState;

pub const MAP_HEIGHT: u32 = 34 * 4;
pub const MAP_WIDTH: u32 = 27 * 4;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(MAP_WIDTH as f32, MAP_HEIGHT as f32)).into(),
        material: materials.add(Color::GREEN),
        ..default()
    });
}
