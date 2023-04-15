mod input;
mod spectator_camera;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PresentMode},
};

use leafwing_input_manager::{prelude::InputManagerPlugin, InputManagerBundle};
use spectator_camera::*;

#[derive(Component)]
struct EscapeMenuTag {
    visible: bool,
}

#[derive(Resource)]
struct CursorLockState(bool);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    });
    // Escape Menu
    setup_escape_menu(commands, asset_server);
}

fn setup_escape_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                display: Display::None,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Auto),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Settings button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Exit button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        })
        .insert(EscapeMenuTag { visible: false });
}

fn set_cursor_lock(mut windows: Query<&mut Window>, cursor_lock_state: Res<CursorLockState>) {
    let mut window = windows.single_mut();
    if cursor_lock_state.0 {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn escape_menu(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Query<&mut Window>,
    mut escape_menu_tag: Query<(&mut EscapeMenuTag, &mut Style)>,
    mut cursor_lock_state: ResMut<CursorLockState>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for (mut escape_menu_tag, mut style) in escape_menu_tag.iter_mut() {
            if escape_menu_tag.visible {
                style.display = Display::None;
                cursor_lock_state.0 = true;
            } else {
                style.display = Display::Flex;
                cursor_lock_state.0 = false;
            }
            escape_menu_tag.visible = !escape_menu_tag.visible;
        }
        set_cursor_lock(windows, cursor_lock_state.into());
    }
}

fn main() {
    App::new()
        .insert_resource(CursorLockState(true))
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
        .add_plugin(InputManagerPlugin::<input::Action>::default())
        .add_startup_system(setup)
        .add_startup_system(set_cursor_lock)
        .add_system(move_camera)
        .add_system(escape_menu)
        .run();
}
