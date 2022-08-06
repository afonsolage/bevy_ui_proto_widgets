use bevy::{prelude::*, ui::FocusPolicy};
use bevy_inspector_egui::WorldInspectorPlugin;
use console::{Console, ConsolePlugin};
use input_text::InputTextPlugin;
use item_list::ItemListPlugin;
use widget::Widget;

mod console;
mod input_text;
mod item_list;
mod widget;

#[derive(Component)]
struct WidgetRoot;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1024.,
        height: 768.,
        ..default()
    });

    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ItemListPlugin)
        .add_plugin(InputTextPlugin)
        .add_plugin(ConsolePlugin)
        .add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    let console = Console::build("Console", &mut commands, &asset_server);

    let _root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
            ..default()
        })
        .insert(WidgetRoot)
        .add_child(console);
}
