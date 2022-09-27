use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Enemy, Monster, Player};

pub fn insert_monster_at(commands: &mut Commands, x: usize, y: usize/* , materials: &Res<Materials> */) {
    let pos = Transform::from_xyz(x as f32, y as f32, 0.);

    let sprite = SpriteBundle {
        sprite: Sprite{
            color: Color::rgb(0.7, 0.9, 0.6),
            custom_size: Some(Vec2::new(0.9, 0.9)),
        ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert(pos)
        .insert(Collider::round_cuboid(0.35, 0.35, 0.1))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(Enemy)
        .insert(Monster);
}

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