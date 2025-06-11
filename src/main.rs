use bevy::{
    color::palettes::css::YELLOW,
    prelude::*,
};

mod plugins;
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const GAMETITLE: &str = "shooting";

#[derive(Component)]
struct Player;
#[derive(Component)]
enum Direction {
    Left,
    Right,
    Upward,
    Downward,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                title: GAMETITLE.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, move_player)
        .add_plugins(plugins::enemy::Enemy)
        .add_plugins(plugins::wall::Wall)
        // .add_systems(Update, move_enemy)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    // 四角形
    commands.spawn((
        Sprite {
            ..Default::default()
        },
        Transform::from_scale(Vec3::new(50.0, 50.0, 1.0)),
    ));
    //黄色い点
    // commands.spawn(
    //     Sprite{
    //         image: asset_server.load("bevy_bird_dark.png"),
    //         color:YELLOW.into(),
    //         ..default()
    // });
    //回る鳥
    commands.spawn((
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
        // Sprite {
        //     image:from_image(asset_server.load("bevy_bird_dark.png")),
        //     ..Default::default()
        // },
        Transform {
            scale: Vec3 {
                x: 0.2,
                y: 0.2,
                z: 0.2,
            },
            translation: Vec3 {
                x: 100.0,
                y: 100.0,
                z: 1.0,
            },
            ..Default::default()
        },
        Direction::Right,
    ));
    //player
    commands.spawn((
        Sprite {
            // image:from_image(asset_server.load("bevy_bird_dark.png")),
            color: YELLOW.into(),
            ..Default::default()
        },
        Transform {
            scale: Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
            translation: Vec3 {
                x: 100.0,
                y: 100.0,
                z: 1.0,
            },
            ..Default::default()
        },
        // Direction::Right,
        Player,
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
        } else if transform.translation.y > 200. && transform.translation.x > -200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. && transform.translation.y > 200. {
            *logo = Direction::Downward;
        } else if transform.translation.y < -200. && transform.translation.x < 200. {
            *logo = Direction::Right;
        }
    }
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let mut delta = Vec3::ZERO;
    let speed = 300.0;

    if keys.pressed(KeyCode::KeyW) {
        delta.y += speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyS) {
        delta.y -= speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyD) {
        delta.x += speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyA) {
        delta.x -= speed * time.delta_secs();
    }

    transform.unwrap().translation += delta;
}
