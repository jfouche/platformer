use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Enemy, Monster};

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