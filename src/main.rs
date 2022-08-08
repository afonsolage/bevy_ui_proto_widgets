use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use button::ButtonClicked;
use console::{Console, ConsoleAction};
use widget::{StringLabel, ToStringLabel, Widget, WidgetEventReader, WidgetLabel, WidgetPlugin};

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
        .add_plugin(WidgetPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(FocusPlugin)
        .add_system(process_toggle_console_btn)
        .add_system(process_string_label_btn)
        .add_startup_system(setup);

    app.run();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ToggleLabel;

// TODO: Convert this into derive later on
impl WidgetLabel for ToggleLabel {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    button::Button::build(ToggleLabel, &mut commands, &asset_server);
    button::Button::build("Skip 1".label(), &mut commands, &asset_server);
    button::Button::build("Skip 2".label(), &mut commands, &asset_server);
    button::Button::build("Skip 3".label(), &mut commands, &asset_server);
    Console::build("Console".label(), &mut commands, &asset_server);
}

fn process_toggle_console_btn(
    mut reader: WidgetEventReader<ToggleLabel, ButtonClicked>,
    mut writer: EventWriter<ConsoleAction>,
) {
    for _evt in reader.iter() {
        writer.send(ConsoleAction::Toggle);
    }
}

fn process_string_label_btn(
    mut reader: WidgetEventReader<StringLabel, ButtonClicked>,
    mut writer: EventWriter<ConsoleAction>,
) {
    for _evt in reader.filter("Skip 1") {
        writer.send(ConsoleAction::Toggle);
    }
}
