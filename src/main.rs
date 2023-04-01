mod spectator_camera;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PresentMode},
};
use spectator_camera::*;

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aetherion".into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_startup_system(toggle_cursor_lock)
        .add_system(move_camera)
        .add_system(escape_menu)
        .run();
}
