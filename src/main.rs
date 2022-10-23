mod player;
mod components;
mod enemy;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::close_on_esc;
use bevy::math::Vec3Swizzles;
use crate::components::{Enemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

// region     --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";

// endregion

// region    --- Asset Constants

const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const SPRITE_SCALE: f32 = 0.5;

// endregion --- Asset Constants

// region       --- Resource

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

pub struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>, // 存储 grid 资源的一种方式
}

// endregion

// region    --- Game Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// endregion --- Game Constants

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(close_on_esc)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>
) {

    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add winSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // create explosion texture atlas
    let texture_handle = assets_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64., 64.),
        4,
        4
    );
    texture_atlases.add(texture_atlas);

    // add game texture resource
    let game_textures = GameTextures {
        player: assets_server.load(PLAYER_SPRITE),
        player_laser: assets_server.load(PLAYER_LASER_SPRITE),
        enemy: assets_server.load(ENEMY_SPRITE),
        enemy_laser: assets_server.load(ENEMY_LASER_SPRITE),
        explosion
    };
    commands.insert_resource(game_textures);

}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            // despawn when out of screen
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
) {
    // iterate through the lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate through the enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determin if collision
            // 确定是否发生碰撞
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale
            );

            // perform collision 执行碰撞
            if let Some(_) = collision {
                // remove the enemy
                commands.entity(enemy_entity).despawn();
                // remove the laser
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
