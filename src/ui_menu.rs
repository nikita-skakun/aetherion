use bevy::{
    app::AppExit,
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowMode},
};
use bevy_egui::{
    egui::{self, Align2, Grid},
    EguiContexts,
};
use bevy_pkv::PkvStore;
use leafwing_input_manager::prelude::ActionState;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    input::Action, menu_focus::CursorLockState, settings::*, settings_io::*,
    spectator_camera::update_fov,
};

const SETTINGS_BUTTON_HEIGHT: f32 = 18.0;

#[derive(Resource, Default)]
pub struct UiVisibility {
    pub escape_menu: bool,
    pub settings_menu: bool,
    pub settings_tab_option: SettingsTabOption,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter, Display, Default)]
pub enum SettingsTabOption {
    #[default]
    General,
    Audio,
    Graphics,
    Controls,
    Debug,
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
    projection_query: Query<&mut Projection>,
    mut ui_visibility: ResMut<UiVisibility>,
    mut cursor_lock_state: ResMut<CursorLockState>,
    mut control_settings: ResMut<ControlSettings>,
    mut graphics_settings: ResMut<GraphicsSettings>,
    mut pkv: ResMut<PkvStore>,
) {
    let action_state = input_query.single_mut();
    let mut window = windows.single_mut();

    if action_state.just_pressed(Action::Exit) {
        let mut escape_used = false;
        if !escape_used && ui_visibility.settings_menu {
            ui_visibility.settings_menu = false;
            escape_used = true;
        }

        //Insert other UI menus here

        if !escape_used {
            ui_visibility.escape_menu = !ui_visibility.escape_menu;
        }

        let ui_window_open = ui_visibility.escape_menu || ui_visibility.settings_menu;
        cursor_lock_state.0 = !ui_window_open;
        set_cursor_lock(&mut window, cursor_lock_state.into());
    }

    if ui_visibility.escape_menu {
        egui::Window::new("Escape Menu")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Settings").clicked() {
                        ui_visibility.settings_menu = true;
                        ui_visibility.escape_menu = false;
                    }
                    if ui.button("Exit").clicked() {
                        app_exit_events.send(AppExit);
                    }
                });
            });
    }

    if ui_visibility.settings_menu {
        egui::Window::new("Settings Menu")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .default_width(window.width() / 3.0)
            .show(contexts.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    for tab_option in SettingsTabOption::iter() {
                        let response = ui.add(egui::Button::new(tab_option.to_string()));

                        if response.clicked() {
                            ui_visibility.settings_tab_option = tab_option;
                        }
                        if tab_option == ui_visibility.settings_tab_option {
                            response.highlight();
                        }
                    }
                });

                ui.separator();

                ui.vertical_centered_justified(|ui| {
                    match ui_visibility.settings_tab_option {
                        SettingsTabOption::General => {
                            ui.label("Nothing here yet :)");
                        }
                        SettingsTabOption::Audio => {
                            ui.label("Nothing here yet :)");
                        }
                        SettingsTabOption::Graphics => {
                            Grid::new("Graphics Settings")
                                .num_columns(2)
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("Window Mode");

                                    if ui
                                        .add_sized(
                                            egui::Vec2::new(
                                                ui.available_width(),
                                                SETTINGS_BUTTON_HEIGHT,
                                            ),
                                            egui::Button::new(match graphics_settings.mode {
                                                WindowMode::Windowed => "Windowed",
                                                WindowMode::BorderlessFullscreen => {
                                                    "Borderless Fullscreen"
                                                }
                                                WindowMode::Fullscreen => "Fullscreen",
                                                _ => "Other?",
                                            }),
                                        )
                                        .clicked()
                                    {
                                        match graphics_settings.mode {
                                            WindowMode::Windowed => {
                                                window.mode = WindowMode::BorderlessFullscreen;
                                                graphics_settings.mode =
                                                    WindowMode::BorderlessFullscreen;
                                            }
                                            WindowMode::BorderlessFullscreen => {
                                                window.mode = WindowMode::Fullscreen;
                                                graphics_settings.mode = WindowMode::Fullscreen;
                                            }
                                            WindowMode::Fullscreen => {
                                                window.mode = WindowMode::Windowed;
                                                graphics_settings.mode = WindowMode::Windowed;
                                            }
                                            _ => {
                                                window.mode = WindowMode::Windowed;
                                                graphics_settings.mode = WindowMode::Windowed;
                                            }
                                        };
                                    };

                                    ui.end_row();

                                    ui.label("Vsync Mode");
                                    if ui
                                        .add_sized(
                                            egui::Vec2::new(
                                                ui.available_width(),
                                                SETTINGS_BUTTON_HEIGHT,
                                            ),
                                            egui::Button::new(match graphics_settings.vsync {
                                                true => "Vsync",
                                                false => "No Vsync",
                                            }),
                                        )
                                        .clicked()
                                    {
                                        match graphics_settings.vsync {
                                            true => {
                                                window.present_mode = PresentMode::AutoNoVsync;
                                                graphics_settings.vsync = false;
                                            }
                                            false => {
                                                window.present_mode = PresentMode::AutoVsync;
                                                graphics_settings.vsync = true;
                                            }
                                        };
                                        export_settings(
                                            &mut *graphics_settings,
                                            "settings.graphics",
                                            &mut pkv,
                                        );
                                    };

                                    ui.end_row();
                                    let mut fov_changed: bool = false;
                                    ui.horizontal_centered(|ui| {
                                        ui.label("Field of View");
                                        if ui
                                            .add_sized(
                                                egui::Vec2::new(40.0, SETTINGS_BUTTON_HEIGHT),
                                                egui::DragValue::new(&mut graphics_settings.fov)
                                                    .clamp_range(30..=100),
                                            )
                                            .changed()
                                        {
                                            fov_changed = true;
                                        };
                                    });
                                    ui.scope(|ui| {
                                        ui.spacing_mut().slider_width = ui.available_width();
                                        if ui
                                            .add(
                                                egui::Slider::new(
                                                    &mut graphics_settings.fov,
                                                    30..=100,
                                                )
                                                .clamp_to_range(true)
                                                .show_value(false),
                                            )
                                            .changed()
                                        {
                                            fov_changed = true;
                                        };
                                    });
                                    if fov_changed {
                                        update_fov(projection_query, &graphics_settings);
                                        export_settings(
                                            &mut *graphics_settings,
                                            "settings.graphics",
                                            &mut pkv,
                                        );
                                    }
                                });
                        }
                        SettingsTabOption::Controls => {
                            //https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/binding_menu.rs
                            Grid::new("Graphics Settings")
                                .num_columns(2)
                                .striped(true)
                                .show(ui, |ui| {
                                    let mut mouse_sensitivity_changed: bool = false;
                                    ui.horizontal_centered(|ui| {
                                        ui.label("Mouse Sensitivity");
                                        if ui
                                            .add_sized(
                                                egui::Vec2::new(40.0, SETTINGS_BUTTON_HEIGHT),
                                                egui::DragValue::new(
                                                    &mut control_settings.mouse_sensitivity,
                                                )
                                                .clamp_range(0.1..=10.0),
                                            )
                                            .changed()
                                        {
                                            mouse_sensitivity_changed = true;
                                        };
                                    });
                                    ui.scope(|ui| {
                                        ui.spacing_mut().slider_width = ui.available_width();
                                        if ui
                                            .add(
                                                egui::Slider::new(
                                                    &mut control_settings.mouse_sensitivity,
                                                    0.1..=10.0,
                                                )
                                                .clamp_to_range(true)
                                                .show_value(false),
                                            )
                                            .changed()
                                        {
                                            mouse_sensitivity_changed = true;
                                        };
                                    });
                                    if mouse_sensitivity_changed {
                                        export_settings(
                                            &mut *control_settings,
                                            "settings.control",
                                            &mut pkv,
                                        );
                                    }
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
