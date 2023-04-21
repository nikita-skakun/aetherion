mod input;
mod menu_focus;
mod settings;
mod settings_io;
mod spectator_camera;
mod ui_menu;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_egui::EguiPlugin;

use leafwing_input_manager::{prelude::InputManagerPlugin, InputManagerBundle};
use menu_focus::CursorLockState;
use settings::*;
use settings_io::*;
use bevy_pkv::PkvStore;
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
    // Sample Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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
                fov: graphics_settings.fov.to_radians(),
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
        .insert_resource(UiVisibility {
            escape_menu: false,
            settings_menu: false,
            settings_tab_option: SettingsTabOption::General,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aetherion".into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(InputManagerPlugin::<input::Action>::default())
        .add_startup_system(import_player_settings)
        .add_startup_system(setup)
        .add_system(move_camera)
        .add_system(ui_menu)
        .run();
}
