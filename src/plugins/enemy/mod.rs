use bevy::prelude::*;

mod enemy;

#[derive(Component)]
pub struct Enemy;

impl Plugin for Enemy {
        fn build(&self, app: &mut App) {
          app.add_systems(Startup, enemy::setup);
                app.add_systems(Update,enemy::move_enemy);
        }
}
