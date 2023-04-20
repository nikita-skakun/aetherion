use bevy::{
    app::AppExit,
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowMode},
};
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};
use leafwing_input_manager::prelude::ActionState;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{input::Action, menu_focus::CursorLockState, spectator_camera::ControlSettings};

#[derive(Resource)]
pub struct UiVisibility {
    pub escape_menu: bool,
    pub settings_menu: bool,
    pub settings_tab_option: SettingsTabOption,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter, Display)]
pub enum SettingsTabOption {
    General,
    Audio,
    Graphics,
    Controls,
    Debug,
}

pub fn setup_ui(mut windows: Query<&mut Window>, cursor_lock_state: Res<CursorLockState>) {
    let mut window = windows.single_mut();
    set_cursor_lock(&mut window, cursor_lock_state);
}

pub fn set_cursor_lock(window: &mut Window, cursor_lock_state: Res<CursorLockState>) {
    if cursor_lock_state.0 {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn ui_menu(
    mut windows: Query<&mut Window>,
    mut contexts: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    mut input_query: Query<&ActionState<Action>, With<Camera3d>>,
    mut visibility: ResMut<UiVisibility>,
    mut cursor_lock_state: ResMut<CursorLockState>,
    mut control_settings: ResMut<ControlSettings>,
) {
    let action_state = input_query.single_mut();
    let mut window = windows.single_mut();

    if action_state.just_pressed(Action::Exit) {
        let mut escape_used = false;
        if !escape_used && visibility.settings_menu {
            visibility.settings_menu = false;
            escape_used = true;
        }

        //Insert other UI menus here

        if !escape_used {
            visibility.escape_menu = !visibility.escape_menu;
        }

        let ui_window_open = visibility.escape_menu || visibility.settings_menu;
        cursor_lock_state.0 = !ui_window_open;
        set_cursor_lock(&mut window, cursor_lock_state.into());
    }

    if visibility.escape_menu {
        egui::Window::new("Escape Menu")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Settings").clicked() {
                        visibility.settings_menu = true;
                        visibility.escape_menu = false;
                    }
                    if ui.button("Exit").clicked() {
                        app_exit_events.send(AppExit);
                    }
                });
            });
    }

    if visibility.settings_menu {
        egui::Window::new("Settings Menu")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            // .default_width(window.width() / 3.0 * 2.0)
            .show(contexts.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    for tab_option in SettingsTabOption::iter() {
                        let tab_button = ui.button(tab_option.to_string());
                        if tab_button.clicked() {
                            visibility.settings_tab_option = tab_option;
                        }
                        if tab_option == visibility.settings_tab_option {
                            tab_button.highlight();
                        }
                    }
                });

                ui.separator();

                ui.vertical_centered_justified(|ui| {
                    match visibility.settings_tab_option {
                        SettingsTabOption::General => {
                            ui.label("Nothing here yet :)");
                        }
                        SettingsTabOption::Audio => {
                            ui.label("Nothing here yet :)");
                        }
                        SettingsTabOption::Graphics => {
                            if ui
                                .button(match window.mode {
                                    WindowMode::Windowed => "Windowed",
                                    WindowMode::BorderlessFullscreen => "Borderless Fullscreen",
                                    WindowMode::Fullscreen => "Fullscreen",
                                    _ => "Other?",
                                })
                                .clicked()
                            {
                                window.mode = match window.mode {
                                    WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                                    WindowMode::BorderlessFullscreen => WindowMode::Fullscreen,
                                    WindowMode::Fullscreen => WindowMode::Windowed,
                                    _ => WindowMode::Windowed,
                                };
                            }

                            if ui
                                .button(match window.present_mode {
                                    PresentMode::AutoVsync => "Vsync",
                                    PresentMode::AutoNoVsync => "No Vsync",
                                    _ => "Other?",
                                })
                                .clicked()
                            {
                                window.present_mode = match window.present_mode {
                                    PresentMode::AutoVsync => PresentMode::AutoNoVsync,
                                    PresentMode::AutoNoVsync => PresentMode::AutoVsync,
                                    _ => PresentMode::AutoVsync,
                                }
                            }
                        }
                        SettingsTabOption::Controls => {
                            //https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/binding_menu.rs
                            ui.horizontal(|ui| {
                                ui.label("Mouse Sensitivity");
                                ui.add(
                                    egui::Slider::new(
                                        &mut control_settings.mouse_sensitivity,
                                        0.1..=10.0,
                                    )
                                    .clamp_to_range(true),
                                );
                            });
                        }
                        SettingsTabOption::Debug => {
                            ui.label("Nothing here yet :)");
                        }
                    };
                });
            });
    }
}
