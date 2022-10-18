mod component;
mod system;

use bevy::prelude::*;
use system::hello::hello_system;

fn main() {
    App::new()
        .add_system(hello_system)
        .run();
}
