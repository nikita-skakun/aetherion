use crate::settings::*;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

fn import_settings<T: Default + serde::de::DeserializeOwned + serde::Serialize>(
    settings: &mut T,
    key: &str,
    pkv: &mut ResMut<'_, PkvStore>,
) {
    if let Ok(new_settings) = pkv.get::<T>(key) {
        *settings = new_settings;
    } else {
        println!("No settings found for key: {}", key);
        export_settings(settings, key, pkv)
    }
}

pub fn export_settings<T: Default + serde::de::DeserializeOwned + serde::Serialize>(
    settings: &mut T,
    key: &str,
    pkv: &mut ResMut<'_, PkvStore>,
) {
    if let Err(e) = pkv.set::<T>(key, settings) {
        println!("Failed to export settings for key {}: {}", key, e);
    }
}

pub fn import_player_settings(
    mut control_settings: ResMut<ControlSettings>,
    mut graphics_settings: ResMut<GraphicsSettings>,
    mut pkv: ResMut<PkvStore>,
) {
    import_settings(&mut *control_settings, "settings.control", &mut pkv);
    import_settings(&mut *graphics_settings, "settings.graphics", &mut pkv);
}
