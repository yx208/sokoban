mod component;
mod system;
mod entity;

use bevy::prelude::*;
use crate::entity::{HelloPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
