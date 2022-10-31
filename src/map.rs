use std::fs::read_to_string;
use bevy::app::{App, Plugin, StartupStage};
use bevy::asset::{Handle};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Image, Res, Transform};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use serde::{Deserialize};

use crate::{GameTextures, GRID_SIZE, MAP_JSON, ROW_COUNT, SPRITE_SCALE};
use crate::components::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_map_system);
    }
}

#[derive(Deserialize, Debug)]
pub struct GameMap(Vec<Vec<u8>>);

fn create_sprite(texture: Handle<Image>, translation: Vec3) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation,
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    }
}

pub fn init_map_system(mut commands: Commands, game_textures: Res<GameTextures>) {

    let map = read_to_string(MAP_JSON).unwrap();
    let map = map.as_str();
    let map: GameMap = serde_json::from_str(map).unwrap();

    // 图片一开始会在中心点，需要把它 offset 到左上角，所以需要除一半的格子数
    let sprite_offset = ROW_COUNT / 2. * GRID_SIZE - GRID_SIZE / 2.;

    for (row_index, row) in map.0.iter().enumerate() {

        // 垂直偏移
        let offset_y = sprite_offset - row_index as f32 * GRID_SIZE;

        for (col_index, col) in row.iter().enumerate()  {

            // 水平偏移
            let offset_x = (col_index as f32) * GRID_SIZE - sprite_offset;

            let translation = Vec3::new(offset_x, offset_y, 1.);
            let position = Position { x: col_index as u8, y: row_index as u8 };

            match col {
                0 => {
                    commands.spawn_bundle(create_sprite(game_textures.floor.clone(), translation))
                        .insert(position)
                        .insert(Floor);
                },
                1 => {
                    commands.spawn_bundle(create_sprite(game_textures.wall.clone(), translation))
                        .insert(position)
                        .insert(Wall)
                        .insert(Immovable);
                },
                2 => {
                    commands.spawn_bundle(create_sprite(game_textures.floor.clone(), translation))
                        .insert(position.clone())
                        .insert(Floor);
                    commands.spawn_bundle(create_sprite(game_textures.spot.clone(), translation))
                        .insert(position)
                        .insert(Spot);
                },
                3 => {
                    commands.spawn_bundle(create_sprite(game_textures.floor.clone(), translation))
                        .insert(position.clone())
                        .insert(Floor);
                    commands.spawn_bundle(create_sprite(game_textures.boxes.clone(), translation))
                        .insert(position)
                        .insert(Movable)
                        .insert(Boxes);
                },
                4 => {
                    commands.spawn_bundle(create_sprite(game_textures.floor.clone(), translation))
                        .insert(position.clone())
                        .insert(Floor);

                    let mut player_pos = translation.clone();
                    player_pos.z = 9.;
                    commands.spawn_bundle(create_sprite(game_textures.player.clone(), player_pos))
                        .insert(position)
                        .insert(Player)
                        .insert(Movable);
                },
                _ => ()
            }
        }
    }
}
