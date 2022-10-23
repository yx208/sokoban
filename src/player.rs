use bevy::prelude::*;
use crate::{BASE_SPEED, GameTextures, Laser, PLAYER_LASER_SIZE, PLAYER_SIZE, PLAYER_SPRITE, SPRITE_SCALE, TIME_STEP, WinSize};
use crate::components::{FromPlayer, Movable, Player, SpriteSize, Velocity};
use crate::CursorIcon::Move;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(create_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    // add player
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
        .insert(Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Movable { auto_despawn: false })
        .insert(Velocity { x: 0., y: 0. });
}

fn player_fire_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(player_kt) = query.get_single() {
        if keyboard.just_pressed(KeyCode::Space) {
            let (x, y) = (player_kt.translation.x, player_kt.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands.spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + 15., 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..default()
                    },
                    ..default()
                })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. });
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

fn create_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if keyboard.pressed(KeyCode::Left) {
            -1.
        } else if keyboard.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}


