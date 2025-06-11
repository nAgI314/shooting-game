use bevy::prelude::*;

mod wall;

#[derive(Component)]
pub struct Wall;


impl Plugin for Wall {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,wall::generate_wall);
    }
}


