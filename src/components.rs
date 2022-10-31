use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8
}

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Immovable;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Spot;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct Boxes;
