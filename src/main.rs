mod input;
mod menu_focus;
mod module;
mod render_utils;
mod settings;
mod settings_io;
mod spectator_camera;
mod ui_menu;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use bevy_pkv::PkvStore;
use leafwing_input_manager::{prelude::InputManagerPlugin, InputManagerBundle};
use menu_focus::CursorLockState;
use module::spawn_module;
use render_utils::update_window;
use settings::*;
use settings_io::*;
use spectator_camera::*;
use ui_menu::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
    graphics_settings: Res<GraphicsSettings>,
    cursor_lock_state: Res<CursorLockState>,
) {
    // Test Module
    let size = Vec3::new(1.0, 1.0, 1.0);
    let position = Vec3::new(0.0, 1.0, 0.0);
    let color = Color::rgb(0.8, 0.7, 0.6);
    spawn_module(
        &mut commands,
        &mut meshes,
        &mut materials,
        size,
        position,
        color,
        false,
    );

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Spectator Camera
    commands.spawn(SpectatorCameraBundle {
        input_manager: InputManagerBundle {
            input_map: SpectatorCameraBundle::default_input_map(),
            ..default()
        },
        camera: Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: f32::to_radians(graphics_settings.fov.into()),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    });

    set_cursor_lock(&mut windows.single_mut(), cursor_lock_state);
}

fn main() {
    App::new()
        .insert_resource(PkvStore::new("aetherion", "game"))
        .insert_resource(CursorLockState(true))
        .insert_resource(ControlSettings::default())
        .insert_resource(GraphicsSettings::default())
        .insert_resource(UiVisibility::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InputManagerPlugin::<input::Action>::default())
        .add_startup_system(import_player_settings)
        .add_startup_system(setup)
        .add_startup_system(update_window)
        .add_system(move_camera)
        .add_system(ui_menu)
        .run();
}
