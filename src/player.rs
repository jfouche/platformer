use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyBuilder};
use crate::{components::*, new_camera_2d};

///
///  spawn player system
/// 
pub fn spawn_player(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {

    let player_size = Vec2::new(1.0, 1.0);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(player_size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player{ speed: 3.5 })
        .insert(Jumper { jump_impulse: 30., is_jumping: false})
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(player_size.x / 2., player_size.y / 2.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::default())
        .with_children(|parent| {
            parent.spawn_bundle(new_camera_2d());
        });
}


///
/// 
/// 
pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>
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
pub fn jump_reset(
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
fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &CollisionEvent) {
    if let CollisionEvent::Started(h1, h2, _) = event {
        if h1 == &entity || h2 == &entity {
            jumper.is_jumping = false
        }
    }
}

///
/// 
/// 
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Velocity)>
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
