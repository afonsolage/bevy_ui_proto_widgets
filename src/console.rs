use bevy::{prelude::*, ui::FocusPolicy};

use crate::{input_text::InputText, item_list::ItemList, widget::Widget};

pub(super) struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Console>().add_system(apply_command);
    }
}

#[derive(Component)]
struct ConsoleMeta {
    command_text: Entity,
    log_items: Entity,
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
        let root = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            color: Color::NONE.into(),
            ..default()
        };

        let panel = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
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

        commands
            .spawn_bundle(root)
            .with_children(|parent| {
                parent
                    .spawn_bundle(panel)
                    .add_child(command_text)
                    .add_child(log_items);
            })
            .insert(Name::new(name))
            .insert(Console::default())
            .insert(ConsoleMeta {
                command_text,
                log_items,
            })
            .id()
    }
}

fn apply_command(
    input: Res<Input<KeyCode>>,
    q: Query<&ConsoleMeta>,
    mut q_input_text: Query<&mut InputText>,
    mut q_item_list: Query<&mut ItemList>,
) {
    if input.just_pressed(KeyCode::Return) == false {
        return;
    }

    for meta in &q {
        let mut input_text = q_input_text
            .get_mut(meta.command_text)
            .expect("Every console should have an input text");

        let cmd = input_text.take();

        let mut item_list = q_item_list
            .get_mut(meta.log_items)
            .expect("Every console should have an item list");

        item_list.items.push(cmd);
    }
}
