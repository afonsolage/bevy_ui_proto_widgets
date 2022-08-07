use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_ui_navigation::{prelude::Focusable, DefaultNavigationPlugins};
use button::ButtonPlugin;
use console::{Console, ConsolePlugin};
use input_text::InputTextPlugin;
use item_list::ItemListPlugin;
use widget::Widget;

mod button;
mod console;
// mod focus;
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
        .add_plugins(DefaultNavigationPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ItemListPlugin)
        .add_plugin(InputTextPlugin)
        .add_plugin(ConsolePlugin)
        .add_plugin(ButtonPlugin)
        // .add_plugin(FocusPlugin)
        .add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    Console::build("Console", &mut commands, &asset_server);
    button::Button::build("Button", &mut commands, &asset_server);
}
