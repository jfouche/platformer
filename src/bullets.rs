use bevy::prelude::*;
use bevy_rapier2d::{prelude::*};

use crate::components::{GameDirection, Bullet};

pub struct BulletOptions {
    pub x: f32,
    pub y: f32,
    pub direction: GameDirection,
}

///
/// 
/// 
pub fn insert_bullet_at(
    commands: &mut Commands,
    // materials: &Res<Materials>,
    options: BulletOptions,
) {
    let speed = match options.direction {
        GameDirection::Left => -14.0,
        _ => 14.0,
    };

    let x = match options.direction {
        GameDirection::Left => options.x - 1.,
        _ => options.x + 1.,
    };
    // let rigid_body = RigidBodyBundle {
    //     position: Vec2::new(x, options.y).into(),
    //     velocity: RigidBodyVelocity {
    //         linvel: Vec2::new(speed, 0.0).into(),
    //         ..Default::default()
    //     },
    //     mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
    //     activation: RigidBodyActivation::cannot_sleep(),
    //     forces: RigidBodyForces {
    //         gravity_scale: 0.,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // };

    // let collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(0.25, 0.05),
    //     flags: ColliderFlags {
    //         active_events: ActiveEvents::CONTACT_EVENTS,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // };

    // let sprite = SpriteBundle {
    //     material: materials.bullet_material.clone(),
    //     sprite: Sprite::new(Vec2::new(0.5, 0.1)),
    //     ..Default::default()
    // };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.0, 0.1),
                custom_size: Some(Vec2::new(0.5, 0.1)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, options.y, 0.),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.25))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(GravityScale(0.001))
        .insert(Velocity {
            linvel: Vec2::new(speed, 0.0),
            angvel: 0.0
        })
        .insert(Bullet);
}