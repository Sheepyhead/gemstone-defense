use crate::{
    map::{Ground, MAP_HEIGHT, MAP_WIDTH},
    GameState,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapCursorPosition::default())
            .add_systems(
                Update,
                (draw_cursor, draw_selection_border).run_if(in_state(GameState::Playing)),
            );
    }
}

fn draw_cursor(
    mut position: ResMut<MapCursorPosition>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation(), Plane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    *position = MapCursorPosition(point);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        point + ground.up() * 0.01,
        Direction3d::new_unchecked(ground.up()), // Up vector is already normalized.
        0.2,
        Color::WHITE,
    );
}

#[derive(Default, Resource)]
pub struct MapCursorPosition(Vec3);

impl MapCursorPosition {
    pub fn to_coords(&self) -> (u32, u32) {
        let x = u32::min(self.0.x as u32, MAP_WIDTH - 1);
        let y = u32::min(self.0.z as u32, MAP_HEIGHT - 1);
        (x, y)
    }
}

fn draw_selection_border(position: Res<MapCursorPosition>, mut gizmos: Gizmos) {
    let (x, z) = position.to_coords();
    gizmos.cuboid(
        Transform::from_xyz(x as f32 + 1., 0.01, z as f32 + 1.).with_scale(Vec3::new(2., 1., 2.)),
        Color::YELLOW,
    )
}
