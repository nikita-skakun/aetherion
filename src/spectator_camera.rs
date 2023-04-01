pub use bevy::{input::mouse::MouseMotion, prelude::*};

const CAMERA_MOVE_SPEED: f32 = 5.0;
const CAMERA_VIEW_SPEED: f32 = 3.0;

pub fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in query.iter_mut() {
        let rotation = transform.rotation;
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation +=
                rotation * Vec3::new(0.0, 0.0, -1.0) * time.delta_seconds() * CAMERA_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation +=
                rotation * Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds() * CAMERA_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation +=
                rotation * Vec3::new(-1.0, 0.0, 0.0) * time.delta_seconds() * CAMERA_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation +=
                rotation * Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds() * CAMERA_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            transform.translation +=
                rotation * Vec3::new(0.0, 1.0, 0.0) * time.delta_seconds() * CAMERA_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::LControl) {
            transform.translation +=
                rotation * Vec3::new(0.0, -1.0, 0.0) * time.delta_seconds() * CAMERA_MOVE_SPEED;
        }
        let (mut delta_x, mut delta_y) = (0.0, 0.0);
        for ev in motion_evr.iter() {
            delta_x += ev.delta.x;
            delta_y += ev.delta.y;
        }
        transform.rotate(Quat::from_rotation_y(-delta_x * CAMERA_VIEW_SPEED * 0.001));
        transform.rotate_local(Quat::from_rotation_x(-delta_y * CAMERA_VIEW_SPEED * 0.001));
    }
}
