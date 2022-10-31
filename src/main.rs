mod player;
mod components;
mod map;

use bevy::prelude::*;
use bevy::window::close_on_esc;
use crate::components::{Boxes, Position, Spot};
use crate::map::{MapPlugin};

use crate::player::{player_movement_event_system};

// region    --- Game Constants

const GRID_SIZE: f32 = 48.;
const ROW_COUNT: f32 = 16.;
const SPRITE_SCALE: f32 = GRID_SIZE / 64.;

// endregion --- Game Constants

// region    --- Asset Constants

const PLAYER_SPRITE: &str = "player.png";
const WALL_SPRITE: &str = "Wall_Brown.png";
const FLOOR_SPRITE: &str = "GroundGravel_Grass.png";
const SPOT_SPRITE: &str = "EndPoint_Brown.png";
const BOX_SPRITE: &str = "Crate_Brown.png";
const MAP_JSON: &str = "src/checkpoint.json";

// endregion --- Asset Constants


// region    --- Game Textures

pub struct GameTextures {
    player: Handle<Image>,
    wall: Handle<Image>,
    floor: Handle<Image>,
    boxes: Handle<Image>,
    spot: Handle<Image>,
}

// endregion --- Game Textures

// region    --- Game Resource

// pub struct WinSize {
//     w: f32,
//     h: f32
// }

// endregion --- Game Resource

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sokoban".to_string(),
            width: GRID_SIZE * ROW_COUNT,
            height: GRID_SIZE * ROW_COUNT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_system(close_on_esc)
        .add_startup_system(startup_system)
        .add_system(player_movement_event_system)
        .add_system(win_check_system)
        .run();
}

fn startup_system(mut commands: Commands, assets_server: Res<AssetServer>) {

    // create 2d camera
    commands.spawn_bundle(Camera2dBundle::default());

    // add game textures resource
    let game_textures = GameTextures {
        player: assets_server.load(PLAYER_SPRITE),
        floor: assets_server.load(FLOOR_SPRITE),
        wall: assets_server.load(WALL_SPRITE),
        boxes: assets_server.load(BOX_SPRITE),
        spot: assets_server.load(SPOT_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn win_check_system(
    boxes_query: Query<&Position, With<Boxes>>,
    spot_query: Query<&Position, With<Spot>>
) {
    let boxes_pos_arr = Vec::from_iter(boxes_query.iter());
    let mut ready: usize = 0;
    for pos in spot_query.iter() {
        if boxes_pos_arr.contains(&pos) {
            ready += 1;
        }
    }

    if ready == boxes_pos_arr.len() {
        println!("You Win");
    }
}
