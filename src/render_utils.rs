use bevy::{prelude::*, window::PresentMode};

use crate::settings::GraphicsSettings;

pub fn update_window(mut windows: Query<&mut Window>, graphics_settings: Res<GraphicsSettings>) {
    let mut window = windows.single_mut();
    window.present_mode = match graphics_settings.vsync {
        true => PresentMode::AutoVsync,
        false => PresentMode::AutoNoVsync,
    };
    window.mode = graphics_settings.mode;
    window.title = "Aetherion".into();
}
