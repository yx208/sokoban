mod component;
mod system;
mod entity;
mod plugin;

use bevy::prelude::*;
use plugin::HelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
