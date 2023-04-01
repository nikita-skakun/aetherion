use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

const CAMERA_MOVE_SPEED: f32 = 5.0;
const CAMERA_VIEW_SPEED: f32 = 3.0;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_camera(
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
        let (mut delta_x, mut delta_y) = (0.0, 0.0);
        for ev in motion_evr.iter() {
            delta_x += ev.delta.x;
            delta_y += ev.delta.y;
        }
        transform.rotate(Quat::from_rotation_y(-delta_x * CAMERA_VIEW_SPEED * 0.001));
        transform.rotate_local(Quat::from_rotation_x(-delta_y * CAMERA_VIEW_SPEED * 0.001));
    }
}

fn toggle_cursor_lock(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = !window.cursor.visible;
    window.cursor.grab_mode = match window.cursor.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
    };
}

fn escape_menu(keyboard_input: Res<Input<KeyCode>>, windows: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        toggle_cursor_lock(windows);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(toggle_cursor_lock)
        .add_system(move_camera)
        .add_system(escape_menu)
        .run();
}
