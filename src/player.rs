use crate::{components::*, new_camera_2d, AppState, /*bullets::{BulletOptions, insert_bullet_at}*/};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

struct PlayerData {
    player_entity: Entity,
    camera_entity: Entity,
}

const PLAYER_SIZE: Vec2 = Vec2::new(1.0, 1.0);

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    player: Player,
    jumper: Jumper,
    body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    constraints: LockedAxes,
    events: ActiveEvents
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.4, 0.5),
                    custom_size: Some(PLAYER_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_xyz(1., 15., 0.),
                ..Default::default()
            },
            player: Player { speed: 8. },
            jumper: Jumper { jump_impulse: 30., is_jumping: false},
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(PLAYER_SIZE.x/2., PLAYER_SIZE.y/2.),
            constraints: LockedAxes::ROTATION_LOCKED,
            events: ActiveEvents::COLLISION_EVENTS,
            velocity: Velocity::default()
        }
    }
}

///
/// PlayerPlugin
/// 
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_jumps)
                    .with_system(player_movement)
                    .with_system(jump_reset)
                    .with_system(death_by_height)
                    .with_system(death_by_enemy)
                    .with_system(fire_controller)
                )
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(cleanup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

///
///  spawn player system
///
fn spawn_player(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(PlayerBundle::default())
        .insert(Name::new("Player"))
        .with_children(|parent| {
            parent
                .spawn_bundle(new_camera_2d())
                .insert(Name::new("Camera"));
        });
}

///
///
///
fn cleanup_player(mut commands: Commands, player_data: Res<PlayerData>) {
    commands
        .entity(player_data.player_entity)
        .despawn_recursive();

    commands
        .entity(player_data.camera_entity)
        .despawn_recursive();
}

///
///
///
fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse);
            jumper.is_jumping = true;
        }
    }
}

///
///
///
fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

///
///
///
fn set_jumping_false_if_touching_floor(
    entity: Entity,
    jumper: &mut Jumper,
    event: &CollisionEvent,
) {
    if let CollisionEvent::Started(h1, h2, _) = event {
        if h1 == &entity || h2 == &entity {
            jumper.is_jumping = false
        }
    }
}

///
///
///
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
        }
    }
}

///
///
///
fn camera_follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.translation.x;
            camera.translation.y = player.translation.y;
        }
    }
}

///
///
///
fn death_by_height(mut commands: Commands, players: Query<(Entity, &Transform), With<Player>>) {
    for (entity, position) in players.iter() {
        if position.translation.y < -1. {
            commands.entity(entity).despawn_recursive();
            warn!("death_by_height");
        }
    }
}

///
///
///
pub fn death_by_enemy(
    mut commands: Commands,
    mut players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = contact_event {
            for player in players.iter_mut() {
                for enemy in enemies.iter() {
                    if (*h1 == player && *h2 == enemy) || (*h1 == enemy && *h2 == player) {
                        commands.entity(player).despawn_recursive();
                    }
                }
            }
        }
    }
}

///
/// 
/// 
pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    materials: Res<Materials>,
    players: Query<(&Player, &Transform), With<Player>>,
) {
    // if keyboard_input.just_pressed(KeyCode::Space) {
    //     for (player, position) in players.iter() {
    //         let options = BulletOptions {
    //             x: position.translation.x,
    //             y: position.translation.y,
    //             direction: GameDirection::Right,
    //         };
    //         insert_bullet_at(&mut commands, /*&materials,*/ options)
    //     }
    // }
}