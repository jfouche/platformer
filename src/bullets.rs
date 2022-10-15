use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Bullet, GameDirection};

const BULLET_SPEED: f32 = 16.0;

pub struct BulletOptions {
    pub x: f32,
    pub y: f32,
    pub direction: GameDirection,
}

#[derive(Bundle)]
struct BulletBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    velocity: Velocity,
    constraints: LockedAxes,
    events: ActiveEvents,
    bullet: Bullet,
}

impl BulletBundle {
    fn new(options: BulletOptions) -> Self {
        let speed = match options.direction {
            GameDirection::Left => -BULLET_SPEED,
            GameDirection::Right => BULLET_SPEED,
        };

        let x = match options.direction {
            GameDirection::Left => options.x - 1.,
            GameDirection::Right => options.x + 1.,
        };
        BulletBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.0, 0.1),
                    custom_size: Some(Vec2::new(0.5, 0.1)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, options.y, 0.),
                ..Default::default()
            },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.25, 0.25),
            gravity: GravityScale(0.01),
            constraints: LockedAxes::ROTATION_LOCKED,
            events: ActiveEvents::CONTACT_FORCE_EVENTS,
            velocity: Velocity::linear(Vec2::new(speed, 0.)),
            bullet: Bullet,
        }
    }
}

///
///
///
pub fn insert_bullet_at(
    commands: &mut Commands,
    // materials: &Res<Materials>,
    options: BulletOptions,
) {
    commands
        .spawn_bundle(BulletBundle::new(options));
}
