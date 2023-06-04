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
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::{prelude::InputManagerPlugin, InputManagerBundle};
use menu_focus::CursorLockState;
use module::{engine_system, spawn_base_module, ModuleEngineTag};
use render_utils::update_window;
use settings::*;
use settings_io::*;
use spectator_camera::*;
use ui_menu::*;

const IS_HEADLESS: bool = true;

/// TODO: Since it is not efficient to have a joint between each module,
/// we could create far away entities to have composite colliders instead,
/// and dynamically switch it out when the player (or other active entity
/// like a bullet) gets close.

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
    mut rapier_config: ResMut<RapierConfiguration>,
    graphics_settings: Res<GraphicsSettings>,
    cursor_lock_state: Res<CursorLockState>,
) {
    // Test Module
    let module1 = spawn_base_module(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Quat::IDENTITY,
        Color::rgb(0.8, 0.2, 0.2),
        Velocity::default(),
        IS_HEADLESS,
    );

    let module2 = spawn_base_module(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        Color::rgb(0.2, 0.8, 0.2),
        Velocity::default(),
        IS_HEADLESS,
    );

    commands
        .entity(module1)
        .insert(ModuleEngineTag { thrust: 1.0 });

    commands
        .entity(module2)
        .insert(ModuleEngineTag { thrust: 1.0 });

    let mut joint_data = FixedJointBuilder::new()
        .local_anchor1(Vec3::new(-0.5, 0.0, 0.0))
        .local_anchor2(Vec3::new(0.5, 0.0, 0.0))
        .build();
    joint_data.set_contacts_enabled(false);

    commands
        .entity(module2)
        .insert(ImpulseJoint::new(module1, joint_data));

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
    rapier_config.gravity = Vec3::ZERO;
}

// fn cast_ray_system(
//     rapier_context: Res<RapierContext>,
//     camera_query: Query<&GlobalTransform, With<Camera3d>>,
// ) {
//     for transform in camera_query.iter() {
//         let max_toi = 4.0;
//         if let Some((entity, toi)) = rapier_context.cast_ray(
//             transform.translation(),
//             transform.forward(),
//             max_toi,
//             true,
//             QueryFilter::default(),
//         ) {
//             // The first collider hit
//             println!("Hit entity: {:?}, toi: {}", entity, toi);
//         }
//     }
// }

fn main() {
    App::new()
        .insert_resource(PkvStore::new("aetherion", "game"))
        .insert_resource(CursorLockState(true))
        .insert_resource(ControlSettings::default())
        .insert_resource(GraphicsSettings::default())
        .insert_resource(UiVisibility::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(InputManagerPlugin::<input::Action>::default())
        .add_startup_system(import_player_settings)
        .add_startup_system(setup)
        .add_startup_system(update_window)
        .add_system(move_camera)
        .add_system(ui_menu)
        .add_system(engine_system)
        .run();
}
