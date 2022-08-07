use std::{borrow::Cow, time::Duration};

use crate::widget::Widget;
use bevy::{prelude::*, ui::FocusPolicy};
use bevy_ui_navigation::prelude::{FocusState, Focusable, Focused};

#[derive(SystemLabel)]
struct RemoveFocus;

pub(super) struct InputTextPlugin;

impl Plugin for InputTextPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InputText>()
            .add_system(remove_focus_when_hidden.label(RemoveFocus))
            .add_system(add_focus_when_shown.after(RemoveFocus))
            .add_system(hide_caret_when_lose_focus.after(RemoveFocus))
            .add_system(update_text_section)
            .add_system(update_text_backspace)
            .add_system(update_text_characters)
            .add_system(update_text_caret);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct InputText {
    text: String,
}

impl InputText {
    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.text)
    }
}

#[derive(Component)]
struct InputTextMeta {
    text_entity: Entity,
    caret_entity: Entity,
    caret_visible: bool,
    caret_timer: Timer,
}

#[derive(Component)]
struct InputTextDisplayText;

#[derive(Component)]
struct InputTextDisplayCaret;

impl Widget for InputText {
    fn build(
        name: impl Into<Cow<'static, str>>,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        let input_panel = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            focus_policy: FocusPolicy::Block,
            color: Color::rgba(0.5, 0.5, 0.5, 0.1).into(),
            ..default()
        };

        let input_text = commands
            .spawn_bundle(TextBundle::from_section(
                "Some fancy command here!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::rgb(0.7, 0.7, 0.7).into(),
                },
            ))
            .insert(InputTextDisplayText)
            .id();

        let input_caret = commands
            .spawn_bundle(
                TextBundle::from_section(
                    "|",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 15.0,
                        color: Color::rgb(0.9, 0.9, 0.9).into(),
                    },
                )
                .with_style(Style {
                    display: Display::None,
                    ..default()
                }),
            )
            .insert(InputTextDisplayCaret)
            .id();

        let panel_bg = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    padding: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(8.0), Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                ..default()
            })
            .add_child(input_text)
            .add_child(input_caret)
            .id();

        commands
            .spawn_bundle(input_panel)
            .add_child(panel_bg)
            .insert(Name::new(name))
            .insert(InputText::default())
            .insert(InputTextMeta {
                text_entity: input_text,
                caret_entity: input_caret,
                caret_visible: false,
                caret_timer: Timer::from_seconds(0.5, true),
            })
            .id()
    }
}

fn remove_focus_when_hidden(
    mut commands: Commands,
    q: Query<
        (Entity, &ComputedVisibility),
        (
            With<InputText>,
            Changed<ComputedVisibility>,
            With<Focusable>,
        ),
    >,
) {
    for (e, visibility) in &q {
        if visibility.is_visible() == false {
            commands.entity(e).remove::<Focusable>();
            commands.entity(e).remove::<Focused>();
        }
    }
}

fn add_focus_when_shown(
    mut commands: Commands,
    q: Query<
        (Entity, &ComputedVisibility),
        (
            With<InputText>,
            Changed<ComputedVisibility>,
            Without<Focusable>,
        ),
    >,
) {
    for (e, visibility) in &q {
        if visibility.is_visible() == true {
            commands.entity(e).insert(Focusable::default());
        }
    }
}

fn hide_caret_when_lose_focus(
    mut q: Query<(&InputTextMeta, &Focusable), (With<InputText>, Changed<Focusable>)>,
    mut q_caret: Query<&mut Style, With<InputTextDisplayCaret>>,
) {
    for (meta, focus) in &mut q {
        if let Ok(mut style) = q_caret.get_mut(meta.caret_entity) {
            if focus.state() == FocusState::Focused && &style.display == &Display::Flex {
                style.display = Display::None;
            }
        }
    }
}

fn update_text_section(
    q: Query<(&InputText, &InputTextMeta), Changed<InputText>>,
    mut q_child: Query<&mut Text, With<InputTextDisplayText>>,
) {
    for (input_text, meta) in &q {
        q_child
            .get_mut(meta.text_entity)
            .expect("Every InputText should have a text child")
            .sections[0]
            .value = input_text.text.clone();
    }
}

fn update_text_characters(
    mut q: Query<(&Focusable, &mut InputText)>,
    mut events: EventReader<ReceivedCharacter>,
) {
    for (focus, mut input_text) in &mut q {
        if focus.state() == FocusState::Focused {
            for evt in events.iter() {
                input_text.text.push(evt.char);
            }
        }
    }
}

fn update_text_backspace(
    mut q: Query<(&Focusable, &mut InputText)>,
    input_keycode: Res<Input<KeyCode>>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    for (focus, mut input_text) in &mut q {
        if focus.state() == FocusState::Focused {
            timer.tick(time.delta());

            let backspace = if input_keycode.pressed(KeyCode::Back) && timer.finished() {
                timer.set_duration(Duration::from_millis(100));
                timer.reset();
                true
            } else if input_keycode.just_pressed(KeyCode::Back) {
                true
            } else {
                false
            };

            if backspace {
                input_text.text.pop();
            }
        }
    }
}

fn update_text_caret(
    mut q: Query<(&Focusable, &mut InputTextMeta), With<InputText>>,
    mut q_caret: Query<&mut Style, With<InputTextDisplayCaret>>,
    time: Res<Time>,
) {
    for (focus, mut meta) in &mut q {
        if focus.state() == FocusState::Focused {
            meta.caret_timer.tick(time.delta());

            if meta.caret_timer.just_finished() {
                let style = &mut q_caret
                    .get_mut(meta.caret_entity)
                    .expect("Every InputText should have a caret child");

                meta.caret_visible = !meta.caret_visible;

                style.display = if meta.caret_visible {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}
