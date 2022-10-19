use crate::{App, Commands, Plugin, Query, With};
use crate::component::{Player, Position};
use crate::system::hello::hello_system;

pub fn add_player(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Position { x: 10, y: 10 });
    commands.spawn().insert(Player).insert(Position { x: 20, y: 20 });
    commands.spawn().insert(Player).insert(Position { x: 30, y: 30 });
}

pub fn greet_player(query: Query<&Position, With<Player>>) {
    for pos in query.iter() {
        // println!("x: {}, y: {}", pos.x, pos.y);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player);
        app.add_system(hello_system);
        app.add_system(greet_player);
    }
}
