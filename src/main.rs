use bevy::{prelude::*, ui::FocusPolicy};
use bevy_inspector_egui::WorldInspectorPlugin;
use input_text::{InputText, InputTextPlugin};
use widget::Widget;

mod input_text;
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
        .add_plugin(InputTextPlugin)
        .add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    let child = build(&mut commands, &*asset_server);

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
        .add_child(child);
}

fn build(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
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

    let log_panel = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            border: UiRect::all(Val::Px(5.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        focus_policy: FocusPolicy::Pass,
        color: Color::rgba(0.1, 0.0, 0.0, 0.9).into(),
        ..default()
    };

    let log_item = TextBundle::from_section(
        "Some thing happened",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 15.0,
            color: Color::rgb(0.6, 0.6, 0.6).into(),
        },
    )
    .with_style(Style {
        size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
        ..default()
    });

    let input_text = InputText::build(commands, asset_server);

    commands
        .spawn_bundle(root)
        .with_children(|parent| {
            parent
                .spawn_bundle(panel)
                .add_child(input_text)
                .with_children(|parent| {
                    parent.spawn_bundle(log_panel).with_children(|parent| {
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                        parent.spawn_bundle(log_item.clone());
                    });
                });
        })
        .id()
}
