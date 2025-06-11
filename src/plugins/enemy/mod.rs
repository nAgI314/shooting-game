use bevy::prelude::*;

mod enemy;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct MyTimer(Timer);


impl Plugin for Enemy {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,enemy::setup);
        app.add_systems(Update, enemy::generate_enemy);
        // app.add_systems(Update, enemy::move_enemy);
        app.add_systems(Update,enemy::apply_velocity);
    }
}


