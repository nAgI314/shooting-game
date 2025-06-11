use bevy::prelude::*;
use rand::Rng;

use crate::plugins::enemy::{Enemy, MyTimer};

#[derive(Component)]
pub struct Velocity(Vec2);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(MyTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

pub fn generate_enemy(mut commands: Commands, mut timer: ResMut<MyTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen_range(0..=360) as f32;

        commands.spawn((
            Sprite {
                ..Default::default()
            },
            Transform::from_scale(Vec3::new(10.0, 10.0, 1.0)),
            Enemy,
            Velocity(Vec2 {
                x: angle.cos(),
                y: angle.sin(),
            }),
        ));
    }
}

pub fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>) {
    let mut transform = query.single_mut();
    let mut delta = Vec3::ZERO;
    let speed = 200.0;

    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0..=360);
    // println!("Random angle: {}", angle);

    transform.unwrap().translation += delta;
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;   
    }
}
