use bevy::{input::mouse::MouseMotion, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::{input::Action, menu_focus::CursorLockState, settings::*};

const CAMERA_MOVE_SPEED: f32 = 5.0;

pub fn move_camera(
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Camera3d>>,
    cursor_lock_state: Res<CursorLockState>,
    control_settings: Res<ControlSettings>,
) {
    if !cursor_lock_state.0 {
        return;
    }
    let (action_state, mut transform) = query.single_mut();
    let rotation = transform.rotation;

    let movement = Vec3::new(
        (action_state.pressed(Action::Right) as i32 - action_state.pressed(Action::Left) as i32)
            as f32,
        (action_state.pressed(Action::Jump) as i32 - action_state.pressed(Action::Crouch) as i32)
            as f32,
        (action_state.pressed(Action::Backward) as i32
            - action_state.pressed(Action::Forward) as i32) as f32,
    );

    transform.translation +=
        rotation * movement.normalize_or_zero() * time.delta_seconds() * CAMERA_MOVE_SPEED;

    let mut delta_x = 0.0;
    let mut delta_y = 0.0;
    for ev in motion_evr.iter() {
        delta_x += ev.delta.x;
        delta_y += ev.delta.y;
    }
    transform.rotate(Quat::from_rotation_y(
        -delta_x * control_settings.mouse_sensitivity * 0.001,
    ));
    transform.rotate_local(Quat::from_rotation_x(
        -delta_y * control_settings.mouse_sensitivity * 0.001,
    ));
}

pub fn update_fov(mut query: Query<&mut Projection>, graphics_settings: &GraphicsSettings) {
    for mut projection in query.iter_mut() {
        if let Projection::Perspective(perspective_projection) = &mut *projection {
            perspective_projection.fov = f32::to_radians(graphics_settings.fov.into());
        }
    }
}

#[derive(Bundle)]
pub struct SpectatorCameraBundle {
    #[bundle]
    pub input_manager: InputManagerBundle<Action>,
    pub camera: Camera3dBundle,
}

impl SpectatorCameraBundle {
    pub fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();

        //UI
        input_map.insert(KeyCode::Escape, Exit);

        //Movement
        input_map.insert(KeyCode::W, Forward);
        input_map.insert(KeyCode::S, Backward);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(KeyCode::Space, Jump);
        input_map.insert(KeyCode::LControl, Crouch);

        //Return
        input_map
    }
}
