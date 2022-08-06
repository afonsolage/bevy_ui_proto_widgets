use std::borrow::Cow;

use bevy::prelude::*;

pub trait Widget {
    fn build(
        name: impl Into<Cow<'static, str>>,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity;
}
