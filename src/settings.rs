use bevy::{prelude::*, window::WindowMode};
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ControlSettings {
    #[serde(default = "default_mouse_sensitivity")]
    pub mouse_sensitivity: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        ControlSettings {
            mouse_sensitivity: default_mouse_sensitivity(),
        }
    }
}

fn default_mouse_sensitivity() -> f32 {
    2.0
}

#[derive(Resource, Debug, Deserialize, Serialize, Clone, Copy)]
pub struct GraphicsSettings {
    #[serde(default = "default_window_mode")]
    pub mode: WindowMode,
    #[serde(default = "default_vsync")]
    pub vsync: bool,
    #[serde(default = "default_fov")]
    pub fov: i8,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        GraphicsSettings {
            fov: default_fov(),
            vsync: default_vsync(),
            mode: default_window_mode(),
        }
    }
}

fn default_window_mode() -> WindowMode {
    WindowMode::Windowed
}

fn default_vsync() -> bool {
    true
}

fn default_fov() -> i8 {
    60
}
