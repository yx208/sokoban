use bevy::input::Input;
use bevy::prelude::{Res, Query, Transform, With, KeyCode, Without};

use crate::components::*;
use crate::{GRID_SIZE};

pub fn player_movement_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query_player: Query<(&mut Transform, &mut Position), With<Player>>,
    mut query_boxes: Query<(&mut Transform, &mut Position), (Without<Player>, With<Boxes>)>,
    query_wall: Query<&Position, (Without<Player>, Without<Boxes>, With<Wall>)>,
) {
    let (mut player_tf, mut player_pos) = query_player.get_single_mut().unwrap();
    let translation = &mut player_tf.translation;

    if keyboard.just_pressed(KeyCode::Left) {

        let wall_arr: Vec<&Position> = query_wall
            .iter()
            .filter(|p| p.x < player_pos.x && p.y == player_pos.y)
            .collect();

        let player_next_pos = Position { x: player_pos.x - 1, y: player_pos.y };

        // 如果下一个是墙，则不进行处理
        if wall_arr.contains(&&player_next_pos) {
            return;
        }

        for (mut box_tf, mut box_pos) in &mut query_boxes.iter_mut() {
            // 是箱子
            if player_next_pos == *box_pos {
                let next_pos = Position { x: player_pos.x - 2, y: player_pos.y };
                if !wall_arr.contains(&&next_pos) {
                    box_tf.translation.x -= GRID_SIZE;
                    box_pos.x -= 1;
                } else {
                    return;
                }
            }
        }

        player_pos.x -= 1;
        translation.x -= GRID_SIZE;
    } else if keyboard.just_pressed(KeyCode::Right) {

        let wall_arr: Vec<&Position> = query_wall
            .iter()
            .filter(|p| p.x > player_pos.x && p.y == player_pos.y)
            .collect();

        let player_next_pos = Position { x: player_pos.x + 1, y: player_pos.y };

        // 如果下一个是墙，则不进行处理
        if wall_arr.contains(&&player_next_pos) {
            return;
        }

        for (mut box_tf, mut box_pos) in &mut query_boxes.iter_mut() {
            // 是箱子
            if player_next_pos == *box_pos {
                let next_pos = Position { x: player_pos.x + 2, y: player_pos.y };
                if !wall_arr.contains(&&next_pos) {
                    box_tf.translation.x += GRID_SIZE;
                    box_pos.x += 1;
                } else {
                    return;
                }
            }
        }

        translation.x += GRID_SIZE;
        player_pos.x += 1;

    } else if keyboard.just_pressed(KeyCode::Up) {

        let wall_arr: Vec<&Position> = query_wall
            .iter()
            .filter(|p| p.x == player_pos.x && p.y < player_pos.y)
            .collect();

        let player_next_pos = Position { x: player_pos.x, y: player_pos.y - 1 };

        // 如果下一个是墙，则不进行处理
        if wall_arr.contains(&&player_next_pos) {
            return;
        }

        for (mut box_tf, mut box_pos) in &mut query_boxes.iter_mut() {
            // 是箱子
            if player_next_pos == *box_pos {
                let next_pos = Position { x: player_pos.x, y: player_pos.y - 2 };
                if !wall_arr.contains(&&next_pos) {
                    box_tf.translation.y += GRID_SIZE;
                    box_pos.y -= 1;
                } else {
                    return;
                }
            }
        }

        player_pos.y -= 1;
        translation.y += GRID_SIZE;
    } else if keyboard.just_pressed(KeyCode::Down) {

        let wall_arr: Vec<&Position> = query_wall
            .iter()
            .filter(|p| p.x == player_pos.x && p.y > player_pos.y)
            .collect();

        let player_next_pos = Position { x: player_pos.x, y: player_pos.y + 1 };

        // 如果下一个是墙，则不进行处理
        if wall_arr.contains(&&player_next_pos) {
            return;
        }

        for (mut box_tf, mut box_pos) in &mut query_boxes.iter_mut() {
            // 是箱子
            if player_next_pos == *box_pos {
                let next_pos = Position { x: player_pos.x, y: player_pos.y + 2 };
                if !wall_arr.contains(&&next_pos) {
                    box_tf.translation.y -= GRID_SIZE;
                    box_pos.y += 1;
                } else {
                    return;
                }
            }
        }

        player_pos.y += 1;
        translation.y -= GRID_SIZE;
    }
}
