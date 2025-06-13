use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::wall::Wall;

const WALL_WIDTH: f32 = 640.0;
const WALL_HEIGHT: f32 = 480.0;
const WALL_THICKNESS: f32 = 20.0;

pub fn generate_wall(mut commands: Commands) {
  //上の壁
    commands.spawn((
        Name::new("wall_up"),
        Sprite{
          custom_size: Some(Vec2::new(WALL_WIDTH, WALL_THICKNESS)),
          ..Default::default()
        },
        Transform{
            translation: Vec3 {
                x: 0.,
                y: WALL_HEIGHT/2.,
                z: 1.0,
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_WIDTH/2., WALL_THICKNESS/2.),
        Wall,
    ));
  //下の壁
    commands.spawn((
        Sprite{
          custom_size: Some(Vec2::new(WALL_WIDTH, WALL_THICKNESS)),
          ..Default::default()
        },
        Transform{
            translation: Vec3 {
                x: 0.,
                y: -WALL_HEIGHT/2.,
                z: 1.0,
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_WIDTH/2., WALL_THICKNESS/2.),
        Wall,
    ));
    //右の壁
    commands.spawn((
        Sprite{
          custom_size: Some(Vec2::new(WALL_THICKNESS, WALL_HEIGHT)),
          ..Default::default()
        },
        Transform{
            translation: Vec3 {
                x: WALL_WIDTH/2.,
                y: 0.,
                z: 1.0,
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS/2., WALL_HEIGHT/2.),
        Wall,
    ));
    //左の壁
    commands.spawn((
        Sprite{
          custom_size: Some(Vec2::new(WALL_THICKNESS, WALL_HEIGHT)),
          ..Default::default()
        },
        Transform{
            translation: Vec3 {
                x: -WALL_WIDTH/2.,
                y: 0.,
                z: 1.0,
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS/2., WALL_HEIGHT/2.),
        Wall,
    ));
}
