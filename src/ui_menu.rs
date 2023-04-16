use bevy::{app::AppExit, prelude::*, window::CursorGrabMode};

use crate::menu_focus::CursorLockState;

#[derive(Component)]
pub struct EscapeMenuTag {
    visible: bool,
}

#[derive(Component)]
pub struct SettingsMenuTag {
    visible: bool,
}

#[derive(Component)]
pub struct EscapeMenuExitButtonTag;

#[derive(Component)]
pub struct EscapeMenuSettingsButtonTag;

fn setup_canvas(commands: &mut Commands) -> bevy::prelude::Entity {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn setup_escape_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, canvas: Entity) {
    let escape_menu_bg = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                display: Display::None,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..Default::default()
        })
        .insert(EscapeMenuTag { visible: false })
        .id();

    commands.entity(canvas).push_children(&[escape_menu_bg]);

    // Settings button
    create_escape_menu_button(
        commands,
        escape_menu_bg,
        "Settings",
        Some(EscapeMenuSettingsButtonTag),
        &asset_server,
    );

    // Exit button
    create_escape_menu_button(
        commands,
        escape_menu_bg,
        "Exit",
        Some(EscapeMenuExitButtonTag),
        &asset_server,
    );
}

fn setup_settings_menu(commands: &mut Commands, canvas: Entity) {
    let settings_menu_bg = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                display: Display::None,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..Default::default()
        })
        .insert(SettingsMenuTag { visible: false })
        .id();
    commands.entity(canvas).push_children(&[settings_menu_bg]);
}

pub fn create_escape_menu_button(
    commands: &mut Commands,
    parent: Entity,
    text: &str,
    tag: Option<impl Component>,
    asset_server: &AssetServer,
) -> Entity {
    let button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                margin: UiRect::all(Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(8.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
        })
        .id();

    if let Some(tag) = tag {
        commands.entity(button).insert(tag);
    }

    commands.entity(parent).push_children(&[button]);
    button
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

pub fn escape_menu(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Query<&mut Window>,
    mut settings_menu_tag: Query<(&mut SettingsMenuTag, &mut Style), Without<EscapeMenuTag>>,
    mut escape_menu_tag: Query<(&mut EscapeMenuTag, &mut Style), Without<SettingsMenuTag>>,
    mut cursor_lock_state: ResMut<CursorLockState>,
    mut app_exit_events: EventWriter<AppExit>,
    settings_button_interaction: Query<
        (&Interaction, &EscapeMenuSettingsButtonTag),
        (Changed<Interaction>, With<Button>),
    >,
    exit_button_interaction: Query<
        (&Interaction, &EscapeMenuExitButtonTag),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let (mut escape_menu_tag, mut escape_menu_style) = escape_menu_tag.single_mut();
    let (mut settings_menu_tag, mut settings_menu_style) = settings_menu_tag.single_mut();

    // Handle escape menu visibility and cursor lock
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if settings_menu_tag.visible {
            settings_menu_style.display = Display::None;
            settings_menu_tag.visible = false;
            escape_menu_style.display = Display::Flex;
            escape_menu_tag.visible = true;
        } else {
            escape_menu_style.display = match escape_menu_tag.visible {
                true => Display::None,
                false => Display::Flex,
            };
            escape_menu_tag.visible = !escape_menu_tag.visible;
            cursor_lock_state.0 = !escape_menu_tag.visible;

            set_cursor_lock(windows, cursor_lock_state.into());
        }
        return;
    }

    if escape_menu_tag.visible {
        // Handle settings button click
        for (interaction, _) in settings_button_interaction.iter() {
            if *interaction == Interaction::Clicked {
                escape_menu_style.display = Display::None;
                escape_menu_tag.visible = false;
                settings_menu_style.display = Display::Flex;
                settings_menu_tag.visible = true;
            }
        }

        // Handle exit button click
        for (interaction, _) in exit_button_interaction.iter() {
            if *interaction == Interaction::Clicked {
                app_exit_events.send(AppExit);
            }
        }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = setup_canvas(&mut commands);
    setup_escape_menu(&mut commands, &asset_server, canvas);
    setup_settings_menu(&mut commands, canvas);
}
