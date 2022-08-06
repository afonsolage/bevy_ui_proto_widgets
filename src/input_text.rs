use bevy::{prelude::*, ui::FocusPolicy};
use bevy_inspector_egui::{
    egui::TextBuffer, widgets::InspectorQuery, Inspectable, InspectorPlugin,
};

use crate::widget::Widget;

#[derive(Inspectable, Default)]
struct InputTextInteraction {
    query: InspectorQuery<Entity, (With<Interaction>, With<InputText>)>,
}

pub(super) struct InputTextPlugin;

impl Plugin for InputTextPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InputText>()
            .init_resource::<InputTextFocused>()
            .add_plugin(InspectorPlugin::<InputTextInteraction>::new())
            .add_system(remove_focus_when_hidden)
            .add_system(set_focus_when_clicked)
            .add_system(update_text_node)
            .add_system(update_text_modifiers)
            .add_system(update_text_characters)
            .add_system(update_text_caret);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct InputText {
    text: String,
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

#[derive(Default)]
pub struct InputTextFocused(pub Option<Entity>);

impl Widget for InputText {
    fn build(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
        let input_panel = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
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
                    padding: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(8.0), Val::Px(8.0)),
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
            .insert(Interaction::default())
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

fn set_focus_when_clicked(
    mut q: Query<(Entity, &Interaction), (With<InputText>, Changed<Interaction>)>,
    mut focused: ResMut<InputTextFocused>,
) {
    for (entity, interaction) in &mut q {
        if interaction == &Interaction::Clicked {
            focused.0 = Some(entity);
        }
    }
}

fn remove_focus_when_hidden(
    q: Query<&ComputedVisibility, (With<InputText>, Changed<ComputedVisibility>)>,
    mut focused: ResMut<InputTextFocused>,
) {
    if let Some(e) = focused.0 {
        if let Ok(computed_visibility) = q.get(e) && computed_visibility.is_visible() == false {
            focused.0 = None
        }
    }
}

fn update_text_node(
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
    mut q: Query<&mut InputText>,
    mut events: EventReader<ReceivedCharacter>,
    focused: ResMut<InputTextFocused>,
) {
    if let Some(e) = focused.0 {
        if let Ok(mut input_text) = q.get_mut(e) {
            for evt in events.iter() {
                input_text.text.push(evt.char);
            }
        }
    }
}

fn update_text_modifiers(
    mut q: Query<&mut InputText>,
    input_keycode: Res<Input<KeyCode>>,
    focused: ResMut<InputTextFocused>,
) {
    if let Some(e) = focused.0 {
        if let Ok(mut input_text) = q.get_mut(e) {
            for keycode in input_keycode.get_just_released() {
                if keycode == &KeyCode::Return || keycode == &KeyCode::NumpadEnter {
                    let value = input_text.text.take();
                    info!("Input value: {}", value);
                } else if keycode == &KeyCode::Back {
                    input_text.text.pop();
                }
            }
        }
    }
}

fn update_text_caret(
    mut q: Query<&mut InputTextMeta, With<InputText>>,
    focused: ResMut<InputTextFocused>,
    mut q_caret: Query<&mut Style, With<InputTextDisplayCaret>>,
    time: Res<Time>,
) {
    if let Some(e) = focused.0 {
        if let Ok(mut meta) = q.get_mut(e) {
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
