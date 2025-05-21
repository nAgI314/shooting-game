use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    Left,
    Right,
    Upward,
    Downward,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup,)
        .add_systems(Update, sprite_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
        Transform::from_xyz(0., 0., 0.),
        Direction::Right,
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 150. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
            Direction::Upward => transform.translation.y += 150. * time.delta_secs(),
            Direction::Downward => transform.translation.y -= 150. * time.delta_secs(),
        }

        if transform.translation.x > 200. && transform.translation.y < 200. {
            *logo = Direction::Upward;
        } else if transform.translation.y > 200. && transform.translation.x > -200.{
            *logo = Direction::Left;
        } else if transform.translation.x < -200. && transform.translation.y > 200.{
            *logo = Direction::Downward;
        } else if transform.translation.y < -200. && transform.translation.x < 200.{
            *logo = Direction::Right;
        }
    }
}