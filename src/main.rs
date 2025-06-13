use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::plugins::wall::Wall;

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

#[derive(Default, Resource)]
pub struct BlockedDirections {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

fn main() {
    App::new()
        .insert_resource(BlockedDirections::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                title: GAMETITLE.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, move_player)
        // .add_systems(Update, player_movement_force)
        .add_plugins(plugins::enemy::Enemy)
        .add_plugins(plugins::wall::Wall)
        // .add_systems(PostUpdate, display_events)
        .add_systems(PostUpdate, handle_collisions)
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
    commands
        .spawn((
            Name::new("player"),
            Sprite {
                // image:from_image(asset_server.load("bevy_bird_dark.png")),
                color: YELLOW.into(),
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            },
            Transform {
                translation: Vec3 {
                    x: 100.0,
                    y: 100.0,
                    z: 1.0,
                },
                ..Default::default()
            },
            // Direction::Right,
            RigidBody::Dynamic,
            Collider::cuboid(5.0, 5.0),
            LockedAxes::ROTATION_LOCKED,
            KinematicCharacterController::default(),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
            Player,
        ))
        .insert(GravityScale(0.0));
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
    // let transform = query.single_mut();
    let mut delta = Vec3::ZERO;
    let speed = 200.0;
    const WINDOW_OFFSET:f32 = 15.;

    let mut transform = match query.single_mut() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Player transform not found: {:?}", e);
            return;
        }
    };

    // println!("{}", transform.translation);
    if keys.pressed(KeyCode::KeyW) && transform.translation[1] < WINDOW_SIZE[1]/2. - WINDOW_OFFSET{
        delta.y += speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyS) && transform.translation[1] > -WINDOW_SIZE[1]/2. + WINDOW_OFFSET{
        delta.y -= speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyD) && transform.translation[0] < WINDOW_SIZE[0]/2. - WINDOW_OFFSET{
        delta.x += speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyA) && transform.translation[0] > -WINDOW_SIZE[0]/2. + WINDOW_OFFSET{
        delta.x -= speed * time.delta_secs();
    }

    // transform.unwrap().translation += delta;
    transform.translation += delta;
}


// fn player_movement( //速度変更型の移動
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     blocked: Res<BlockedDirections>,
//     mut player_info: Query<(&Player, &mut Velocity)>,
// ) {
//     for (_player, mut rb_vels) in &mut player_info {
//         let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) && !blocked.up;
//         let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) && !blocked.down;
//         let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) && !blocked.left;
//         let right =
//             keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) && !blocked.right;

//         let x_axis = -(left as i8) + right as i8;
//         let y_axis = -(down as i8) + up as i8;

//         let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
//         if move_delta != Vec2::ZERO {
//             move_delta = move_delta.normalize() * 200.0;
//         }

//         rb_vels.linvel = move_delta;
//     }
// }

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<&Transform, With<Player>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    mut blocked: ResMut<BlockedDirections>,
) {
    for event in collision_events.read() {
        println!("Received collision event: {event:?}");
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (player_entity, wall_entity) =
                if player_query.get(*e1).is_ok() && wall_query.get(*e2).is_ok() {
                    (*e1, *e2)
                } else if player_query.get(*e2).is_ok() && wall_query.get(*e1).is_ok() {
                    (*e2, *e1)
                } else {
                    continue;
                };

            let player_pos = player_query.get(player_entity).unwrap().translation;
            let wall_pos = wall_query.get(wall_entity).unwrap().translation;
            let delta = wall_pos - player_pos;

            if delta.x.abs() > delta.y.abs() {
                if delta.x > 0.0 {
                    blocked.right = true;
                } else {
                    blocked.left = true;
                }
            } else {
                if delta.y > 0.0 {
                    blocked.up = true;
                } else {
                    blocked.down = true;
                }
            }
        }

        if let CollisionEvent::Stopped(_, _, _) = event {
            *blocked = BlockedDirections::default();
        }
    }
}
