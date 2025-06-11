use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::wall::Wall;

const WALL_WIDTH: f32 = 640.0;
const WALL_HEIGHT: f32 = 480.0;
const WALL_THICKNESS: f32 = 10.0;

pub fn generate_wall(mut commands: Commands) {
  //上の壁
    commands.spawn((
        Sprite{
          ..Default::default()
        },
        Transform{
          scale: Vec3 {
                x: WALL_WIDTH,
                y: WALL_THICKNESS,
                z: 1.0,
            },
            translation: Vec3 {
                x: 0.,
                y: WALL_HEIGHT/2.,
                z: 1.0,
            },
            ..Default::default()
        },
        Wall,
    ));
  //下の壁
    commands.spawn((
        Sprite{
          ..Default::default()
        },
        Transform{
          scale: Vec3 {
                x: WALL_WIDTH,
                y: WALL_THICKNESS,
                z: 1.0,
            },
            translation: Vec3 {
                x: 0.,
                y: -WALL_HEIGHT/2.,
                z: 1.0,
            },
            ..Default::default()
        },
        Wall,
    ));
    //右の壁
    commands.spawn((
        Sprite{
          ..Default::default()
        },
        Transform{
          scale: Vec3 {
                x: WALL_THICKNESS,
                y: WALL_HEIGHT,
                z: 1.0,
            },
            translation: Vec3 {
                x: WALL_WIDTH/2.,
                y: 0.,
                z: 1.0,
            },
            ..Default::default()
        },
        Wall,
    ));
    //左の壁
    commands.spawn((
        Sprite{
          ..Default::default()
        },
        Transform{
          scale: Vec3 {
                x: WALL_THICKNESS,
                y: WALL_HEIGHT,
                z: 1.0,
            },
            translation: Vec3 {
                x: -WALL_WIDTH/2.,
                y: 0.,
                z: 1.0,
            },
            ..Default::default()
        },
        Wall,
    ));
}
