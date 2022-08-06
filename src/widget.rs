use bevy::prelude::*;

pub trait Widget {
    fn build(commands: &mut Commands, asset_server: &AssetServer) -> Entity;
}