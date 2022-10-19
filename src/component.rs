use bevy::prelude::Component;

/// 可移动
#[derive(Component)]
pub struct Movable;

/// 不可移动
#[derive(Component)]
pub struct Immovable;

/// 位置
#[derive(Component, Default)]
pub struct Position {
    pub(crate) x: u32,
    pub(crate) y: u32
}

/// 墙
#[derive(Component)]
pub struct Wall;

/// 地板
#[derive(Component)]
pub struct Floor;

/// 箱子
#[derive(Component)]
pub struct AnBox;

/// 箱子目标点
#[derive(Component)]
pub struct BoxSpot;

/// 玩家
#[derive(Component)]
pub struct Player;
