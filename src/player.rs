use crate::{components::*, new_camera_2d};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

struct PlayerData {
    player_entity: Entity,
    camera_entity: Entity,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("player_setup", SystemStage::single(spawn_player))
            // .add_system(camera_follow_player)
            .add_system(player_jumps)
            .add_system(player_movement)
            .add_system(jump_reset)
            .add_system(death_by_height)
            // .add_system(death_by_enemy)
            ;
    }
}

///
///  spawn player system
///
fn spawn_player(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {
    let player_size = Vec2::new(1.0, 1.0);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.4, 0.5),
                custom_size: Some(player_size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 8. })
        .insert(Jumper {
            jump_impulse: 30.,
            is_jumping: false,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(player_size.x / 2., player_size.y / 2.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::default())
        .insert(Transform::from_xyz(1., 15., 0.))
        .with_children(|parent| {
            parent.spawn_bundle(new_camera_2d());
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
        }
    }
}
