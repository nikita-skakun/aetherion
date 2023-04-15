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

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
                        size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Escape Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        }),
                    );
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
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }
}

fn escape_menu(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Query<&mut Window>,
    mut escape_menu_tag: Query<(&EscapeMenuTag, &mut Style)>,
    mut cursor_lock_state: ResMut<CursorLockState>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for (_, mut style) in escape_menu_tag.iter_mut() {
            if let Display::None = style.display {
                style.display = Display::Flex;
                cursor_lock_state.0 = false;
            } else {
                style.display = Display::None;
                cursor_lock_state.0 = true;
            }
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
