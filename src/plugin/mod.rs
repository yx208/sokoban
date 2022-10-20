use crate::{App, Commands, Plugin, Timer};
use crate::component::{Player, Position};
use crate::entity::{greet_player, GreetTimer};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
        app.add_startup_system(add_player);
        app.add_system(greet_player);
    }
}

fn add_player(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Position { x: 10, y: 10 });
    commands.spawn().insert(Player).insert(Position { x: 20, y: 20 });
    commands.spawn().insert(Player).insert(Position { x: 30, y: 30 });
}
