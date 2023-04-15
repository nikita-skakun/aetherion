use bevy::{app::AppExit, prelude::*, window::CursorGrabMode};

use crate::menu_focus::CursorLockState;

#[derive(Component)]
pub struct EscapeMenuTag {
    visible: bool,
}

#[derive(Component)]
pub struct EscapeMenuExitButtonTag;

#[derive(Component)]
pub struct EscapeMenuSettingsButtonTag;

pub fn setup_escape_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EscapeMenuTag { visible: false })
        .id();

    let escape_menu_bg = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..Default::default()
        })
        .id();

    commands.entity(canvas).push_children(&[escape_menu_bg]);

    // Settings button
    create_escape_menu_button(
        &mut commands,
        escape_menu_bg,
        canvas,
        "Settings",
        Some(EscapeMenuSettingsButtonTag),
        &asset_server,
    );

    // Exit button
    create_escape_menu_button(
        &mut commands,
        escape_menu_bg,
        canvas,
        "Exit",
        Some(EscapeMenuExitButtonTag),
        &asset_server,
    );
}

pub fn create_escape_menu_button(
    commands: &mut Commands,
    menu_background: Entity,
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
    commands.entity(menu_background).push_children(&[button]);
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
    mut escape_menu_tag: Query<(&mut EscapeMenuTag, &mut Style)>,
    mut cursor_lock_state: ResMut<CursorLockState>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for (mut escape_menu_tag, mut style) in escape_menu_tag.iter_mut() {
            style.display = match escape_menu_tag.visible {
                true => Display::None,
                false => Display::Flex,
            };
            escape_menu_tag.visible = !escape_menu_tag.visible;
            cursor_lock_state.0 = !escape_menu_tag.visible;
        }
        set_cursor_lock(windows, cursor_lock_state.into());
    }
}

pub fn exit_button_system(
    mut app_exit_events: EventWriter<AppExit>,
    button_interaction_query: Query<
        (&Interaction, &EscapeMenuExitButtonTag),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, _) in button_interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            app_exit_events.send(AppExit);
        }
    }
}
