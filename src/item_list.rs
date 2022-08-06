use bevy::{prelude::*, ui::FocusPolicy};

use crate::widget::Widget;

pub(super) struct ItemListPlugin;

impl Plugin for ItemListPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemList>()
            .register_type::<ItemIndex>()
            .add_system(update_item_list_items);
    }
}

#[derive(Component)]
struct ItemListMeta {
    container_entity: Entity,
    item_font: Handle<Font>,
}

impl ItemListMeta {
    fn create_item_bundle(&self, content: String) -> TextBundle {
        TextBundle::from_section(
            content,
            TextStyle {
                font: self.item_font.clone(),
                font_size: 15.0,
                color: Color::rgb(0.6, 0.6, 0.6).into(),
            },
        )
        .with_style(Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            ..default()
        })
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct ItemIndex(usize);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ItemList {
    items: Vec<String>,
}

impl Widget for ItemList {
    fn build(
        name: impl Into<std::borrow::Cow<'static, str>>,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        let list_bg = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                ..default()
            })
            .id();

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.5, 0.5, 0.5, 0.1).into(),
                ..default()
            })
            .add_child(list_bg)
            .insert(Name::new(name))
            .insert(ItemList::default())
            .insert(ItemListMeta {
                container_entity: list_bg,
                item_font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            })
            .id()
    }
}

fn update_item_list_items(
    mut commands: Commands,
    q: Query<(&Children, &ItemList, &ItemListMeta), (With<ItemListMeta>, Changed<ItemList>)>,
    mut q_items: Query<(Entity, &mut Text), With<ItemIndex>>,
) {
    for (children, item_list, meta) in &q {
        // Sync children with item list items
        for (index, item) in item_list.items.iter().enumerate() {
            let item_entity = if index <= children.len() {
                let (entity, mut text) = q_items
                    .get_mut(children[index])
                    .expect("Child item should exists");
                text.sections[0].value = item.clone();
                entity
            } else {
                commands
                    .spawn_bundle(meta.create_item_bundle(item.clone()))
                    .id()
            };

            commands
                .entity(item_entity)
                .insert(ItemIndex(index))
                .insert(Name::new(format!("Item {index}")));

            commands
                .entity(meta.container_entity)
                .add_child(item_entity);
        }

        // Remove unused children
        for i in (item_list.items.len() - 1)..children.len() {
            commands.entity(children[i]).despawn();
        }
    }
}
