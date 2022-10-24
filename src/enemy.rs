use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::{Rng, thread_rng};

use crate::{ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, EnemyCount, FromEnemy, GameTextures, Laser, Movable, SPRITE_SCALE, Velocity, WinSize};
use crate::components::{Enemy, SpriteSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(enemy_spawn_system)
        )
            .add_system(enemy_fire_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {

    if enemy_count.0 >= ENEMY_MAX {
        return;
    }

    // compute x/y
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
        .insert(Enemy)
        .insert(SpriteSize::from(ENEMY_SIZE));

    enemy_count.0 += 1;
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for &tf in enemy_query.iter() {

        let (x, y) = (tf.translation.x, tf.translation.y);

        // spawn enemy laser sprite
        commands.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy_laser.clone(),
            transform: Transform {
                translation: Vec3::new(x, y - 15., 0.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0., y: -1. });
    }
}
