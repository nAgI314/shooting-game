use bevy::prelude::*;
use rand::Rng;


use crate::plugins::enemy::Enemy;



pub fn setup(mut commands: Commands) {
    commands.spawn((
        Sprite {
            ..Default::default()
        },
        Transform::from_scale(Vec3::new(10.0, 10.0, 1.0)),
        Enemy,
    ));
}

pub fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>,) {
  let mut transform = query.single_mut();
  let mut delta = Vec3::ZERO;
  let speed = 200.0;

  let mut rng = rand::thread_rng();
  let angle= rng.gen_range(0..=360);
  // println!("Random angle: {}", angle);
}
