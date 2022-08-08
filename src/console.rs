use bevy::{prelude::*, ui::FocusPolicy};
use bevy_ui_navigation::prelude::NavRequest;

use crate::{input_text::InputText, item_list::ItemList, widget::Widget};

const CONSOLE_HEIGHT_PERC: f32 = 80.0;
const CONSOLE_ANIMATION_SPEED: f32 = 250.0;

pub(super) struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Console>()
            .add_system(apply_command)
            .add_system(console_animation)
            .add_system(toggle_console);
    }
}

struct ConsoleMeta {
    entity: Entity,
    command_text: Entity,
    log_items: Entity,
    direction: i8,
    visible: bool,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Console;

impl Widget for Console {
    fn build(
        name: impl Into<std::borrow::Cow<'static, str>>,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        let panel = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(CONSOLE_HEIGHT_PERC)),
                position: UiRect::new(
                    Val::Undefined,
                    Val::Undefined,
                    Val::Percent(-CONSOLE_HEIGHT_PERC),
                    Val::Undefined,
                ),
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexEnd,
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        };

        let command_text = InputText::build("command_text", commands, asset_server);
        let log_items = ItemList::build("log_items", commands, asset_server);

        let entity = commands
            .spawn_bundle(panel)
            .add_child(command_text)
            .add_child(log_items)
            .insert(Name::new(name))
            .insert(Console::default())
            .insert(Visibility { is_visible: false })
            .id();

        commands.insert_resource(ConsoleMeta {
            command_text,
            log_items,
            direction: 0,
            visible: false,
            entity,
        });

        entity
    }
}

fn toggle_console(mut meta: ResMut<ConsoleMeta>, input: Res<Input<KeyCode>>) {
    if input.any_just_pressed([KeyCode::Grave, KeyCode::Apostrophe])
        && input.pressed(KeyCode::LControl)
    {
        if meta.visible && meta.direction == 0 {
            meta.direction = -1;
        } else if meta.visible == false && meta.direction == 0 {
            meta.direction = 1;
        }
    }
}

fn apply_command(
    input: Res<Input<KeyCode>>,
    meta: Res<ConsoleMeta>,
    mut q_input_text: Query<&mut InputText>,
    mut q_item_list: Query<&mut ItemList>,
) {
    if input.just_pressed(KeyCode::Return) == false {
        return;
    }

    let mut input_text = q_input_text
        .get_mut(meta.command_text)
        .expect("Every console should have an input text");

    let cmd = input_text.take();

    let mut item_list = q_item_list
        .get_mut(meta.log_items)
        .expect("Every console should have an item list");

    item_list.items.push(cmd);
}

fn console_animation(
    mut q: Query<(&mut Style, &mut Visibility), With<Console>>,
    time: Res<Time>,
    mut meta: ResMut<ConsoleMeta>,
    mut writer: EventWriter<NavRequest>,
) {
    if let Ok((mut style, mut visibility)) = q.get_mut(meta.entity) {
        if meta.direction == 0 {
            return;
        } else {
            let mut top = match style.position.top {
                Val::Percent(top) => top,
                _ => unreachable!(),
            };

            top += meta.direction as f32 * time.delta_seconds() * CONSOLE_ANIMATION_SPEED;

            if meta.direction == 1 && top >= 0.0 {
                style.position.top = Val::Percent(0.0);
                meta.direction = 0;
                meta.visible = true;

                writer.send(NavRequest::FocusOn(meta.command_text));
            } else if meta.direction == -1 && top <= -CONSOLE_HEIGHT_PERC {
                style.position.top = Val::Percent(-CONSOLE_HEIGHT_PERC);
                meta.direction = 0;
                meta.visible = false;
            } else {
                style.position.top = Val::Percent(top);
            }

            if top <= -CONSOLE_HEIGHT_PERC {
                visibility.is_visible = false;
            } else if visibility.is_visible == false && top > -CONSOLE_HEIGHT_PERC {
                visibility.is_visible = true;
            }
        }
    }
}
