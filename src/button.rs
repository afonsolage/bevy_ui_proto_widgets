use bevy::{prelude::*, ui::FocusPolicy};

use crate::{focus::Focus, widget::Widget};

const NORMAL_COLOR: Color = Color::NONE;
const HOVERED_COLOR: Color = Color::rgba(0.8, 0.8, 0.8, 0.3);
const CLICKED_COLOR: Color = Color::rgba(0.05, 0.05, 0.05, 0.5);

pub(super) struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Button>().add_system(update_color);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Button;

#[derive(Component)]
struct ButtonMask;

#[derive(Component)]
struct ButtonMeta {
    mask: Entity,
}

impl Widget for Button {
    fn build(
        name: impl Into<std::borrow::Cow<'static, str>>,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        let text = commands
            .spawn_bundle(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ))
            .id();

        let bg = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                ..default()
            })
            .add_child(text)
            .id();

        let border = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.5, 0.5, 0.5, 0.1).into(),
                ..default()
            })
            .add_child(bg)
            .id();

        let mask = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.0, 0.9, 0.0, 0.3).into(),
                ..default()
            })
            .insert(ButtonMask)
            .insert(Name::new("Mask"))
            .id();

        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position: UiRect::new(
                        Val::Undefined,
                        Val::Undefined,
                        Val::Undefined,
                        Val::Px(-300.0),
                    ),
                    size: Size::new(Val::Px(100.0), Val::Px(40.0)),
                    // center button
                    margin: UiRect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new(name))
            .insert(Focus::default())
            .insert(Interaction::default())
            .insert(Button)
            .insert(ButtonMeta { mask })
            .add_child(border)
            .add_child(mask)
            .id()
    }
}

fn update_color(
    mut q_mask: Query<&mut UiColor, With<ButtonMask>>,
    q: Query<(&ButtonMeta, &Interaction), Changed<Interaction>>,
) {
    for (meta, interaction) in &q {
        if let Ok(mut color) = q_mask.get_mut(meta.mask) {
            color.0 = match interaction {
                Interaction::Clicked => CLICKED_COLOR,
                Interaction::Hovered => HOVERED_COLOR,
                Interaction::None => NORMAL_COLOR,
            };
        }
    }
}