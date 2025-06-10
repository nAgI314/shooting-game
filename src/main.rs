use bevy::{color::palettes::css::YELLOW, input::{keyboard::KeyboardInput, ButtonState}, prelude::*};

const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const GAMETITLE: &str = "shooting";

#[derive(Component)]
enum Direction {
    Left,
    Right,
    Upward,
    Downward,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )    
        .add_systems(Startup, setup,)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, keyboard_events)
        .add_systems(Update,move_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        Camera2d::default(),
    );
    // 四角形
    commands.spawn((
        Sprite {    
            ..Default::default()
        },
        Transform::from_scale(Vec3::new(100.0, 100.0, 1.0)),
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
        Transform {
            scale: Vec3{
                x: 0.2,
                y: 0.2,
                z: 0.2},
            translation: Vec3 { 
                x: 100.0, 
                y: 100.0, 
                z: 1.0},
            ..default()
        },
        
        Direction::Right,
    ));
    commands.spawn((
        Sprite{
            // image:from_image(asset_server.load("bevy_bird_dark.png")),
            color:YELLOW.into(),
            ..Default::default()},
        Transform {
            scale: Vec3{
                x: 10.0,
                y: 10.0,
                z: 10.0},
            translation: Vec3 { 
                x: 100.0, 
                y: 100.0, 
                z: 1.0},
            ..default()
        },
        // Direction::Right,
    ));
//     commands.spawn(PointLight {
//         color:YELLOW.into(),
//         intensity: 1000.0,
//     ..default()
// });
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

fn keyboard_events(
    mut evr_kbd: EventReader<KeyboardInput>,
) {
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({:?})", ev.key_code, ev.logical_key);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({:?})", ev.key_code, ev.logical_key);
            }
        }
    }
}

fn move_player(){}