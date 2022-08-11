use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use button::{ButtonClicked, TextButton};
use console::{CommandIssued, Console, ConsoleAction};
use widget::{Widget, WidgetEventReader, WidgetLabel, WidgetPlugin};

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
        .add_system(process_console_cmd)
        .add_startup_system(setup);

    app.run();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ToggleButton;

// TODO: Convert this into derive later on
impl WidgetLabel for ToggleButton {}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct LogConsole;
// TODO: Convert this into derive later on
impl WidgetLabel for LogConsole {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    TextButton::build(ToggleButton, &mut commands, &asset_server);
    Console::build(LogConsole, &mut commands, &asset_server);
}

fn process_toggle_console_btn(
    mut reader: WidgetEventReader<ToggleButton, ButtonClicked>,
    mut writer: EventWriter<ConsoleAction>,
) {
    for _evt in reader.iter() {
        writer.send(ConsoleAction::Toggle);
    }
}

fn process_console_cmd(mut reader: WidgetEventReader<LogConsole, CommandIssued>) {
    for CommandIssued {
        0: _entity,
        1: cmd,
    } in reader.iter()
    {
        info!("Received console cmd: {}", cmd);
    }
}
