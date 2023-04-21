use bevy::prelude::*;
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
    #[serde(default = "default_fov")]
    pub fov: f32,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        GraphicsSettings { fov: default_fov() }
    }
}

fn default_fov() -> f32 {
    60.0
}
