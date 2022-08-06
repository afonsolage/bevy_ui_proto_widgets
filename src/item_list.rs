use bevy::{prelude::*, ui::FocusPolicy};

use crate::widget::Widget;

const ITEM_HEIGHT: f32 = 20.0;

pub(super) struct ItemListPlugin;

impl Plugin for ItemListPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemList>()
            .register_type::<ItemIndex>()
            .add_system(update_item_list_items)
            .add_system(update_item_list_max_visible_items);
    }
}

#[derive(Component)]
struct ItemListMeta {
    container_entity: Entity,
    item_font: Handle<Font>,
    max_visible_items: usize,
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
            flex_shrink: 0.0,
            size: Size::new(Val::Percent(100.0), Val::Px(ITEM_HEIGHT)),
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
    pub items: Vec<String>,
}

#[derive(Component)]
struct ItemListContainer;

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
                    flex_shrink: 0.0,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                ..default()
            })
            .insert(ItemListContainer)
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
                max_visible_items: 0,
            })
            .id()
    }
}

fn update_item_list_items(
    mut commands: Commands,
    q: Query<(&ItemList, &ItemListMeta), (With<ItemListMeta>, Changed<ItemList>)>,
    q_containers: Query<&Children, With<ItemListContainer>>,
    mut q_items: Query<(Entity, &mut Text), With<ItemIndex>>,
) {
    for (item_list, meta) in &q {
        let children = q_containers.get(meta.container_entity).ok();

        // Sync children with item list items
        for (index, item) in item_list.items.iter().rev().enumerate() {
            if index >= meta.max_visible_items {
                break;
            }

            let item_entity = if let Some(children) = children && index < children.len() {
                let (entity, mut text) = q_items
                    .get_mut(children[index])
                    .expect("Child item should exists");
                text.sections[0].value = item.clone();
                entity
            } else {
                let item = commands
                    .spawn_bundle(meta.create_item_bundle(item.clone()))
                    .id();
                commands.entity(meta.container_entity).add_child(item);
                item
            };

            commands
                .entity(item_entity)
                .insert(ItemIndex(index))
                .insert(Name::new(format!("Item {index}")));
        }

        // Remove unused children
        if let Some(children) = children {
            for i in (item_list.items.len() - 1)..children.len() {
                commands.entity(children[i]).despawn();
            }
        }
    }
}

fn update_item_list_max_visible_items(
    mut q: Query<&mut ItemListMeta, (With<ItemList>, Changed<Node>)>,
    q_containers: Query<&Node, With<ItemListContainer>>,
) {
    for mut meta in &mut q {
        if let Ok(container_node) = q_containers.get(meta.container_entity) {
            meta.max_visible_items = (container_node.size.y / ITEM_HEIGHT) as usize;
        }
    }
}
