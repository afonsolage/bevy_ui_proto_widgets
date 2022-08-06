use bevy::{prelude::*, ui::FocusPolicy};

use crate::{input_text::InputText, item_list::ItemList, widget::Widget};

pub(super) struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Console>();
    }
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

        let input_text = InputText::build("command_text", commands, asset_server);
        let log_items = ItemList::build("log_items", commands, asset_server);

        commands
            .spawn_bundle(root)
            .with_children(|parent| {
                parent
                    .spawn_bundle(panel)
                    .add_child(input_text)
                    .add_child(log_items);
            })
            .insert(Console::default())
            .insert(Name::new(name))
            .id()
    }
}
