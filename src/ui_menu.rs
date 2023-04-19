use bevy::{app::AppExit, prelude::*, window::CursorGrabMode};
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};
use leafwing_input_manager::prelude::ActionState;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{input::Action, menu_focus::CursorLockState};

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

pub fn set_cursor_lock(mut windows: Query<&mut Window>, cursor_lock_state: Res<CursorLockState>) {
    let mut window = windows.single_mut();
    if cursor_lock_state.0 {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn ui_menu(
    windows: Query<&mut Window>,
    mut contexts: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    mut input_query: Query<&ActionState<Action>, With<Camera3d>>,
    mut visibility: ResMut<UiVisibility>,
    mut cursor_lock_state: ResMut<CursorLockState>,
) {
    let action_state = input_query.single_mut();

    if action_state.just_pressed(Action::Exit) {
        let mut escape_used = false;
        if !escape_used && visibility.settings_menu {
            visibility.settings_menu = false;
            visibility.escape_menu = true;
            escape_used = true;
        }

        //Insert other UI menus here

        if !escape_used {
            visibility.escape_menu = !visibility.escape_menu;
        }

        let ui_window_open = visibility.escape_menu || visibility.settings_menu;
        cursor_lock_state.0 = !ui_window_open;
        set_cursor_lock(windows, cursor_lock_state.into());
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

                ui.vertical_centered_justified(|ui| {
                    match visibility.settings_tab_option {
                        SettingsTabOption::General => {}
                        SettingsTabOption::Audio => {}
                        SettingsTabOption::Graphics => {}
                        SettingsTabOption::Controls => {}
                        SettingsTabOption::Debug => {
                            ui.label("Nothing here yet :)");
                        }
                    };
                });
            });
    }
}
