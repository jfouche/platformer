use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Enemy, Monster};

const MONSTER_SIZE: Vec2 = Vec2::new(0.9, 0.9);

#[derive(Bundle)]
struct MonsterBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
    body: RigidBody,
    constraints: LockedAxes,
    events: ActiveEvents,
    enemy: Enemy,
    monster: Monster
}

impl MonsterBundle {
    ///
    /// Create a new Monster Bundle for a monster at pos (x, y)
    /// 
    fn new(x: f32, y: f32) -> Self {
        MonsterBundle {  
            sprite_bundle: SpriteBundle {
                sprite: Sprite{
                    color: Color::rgb(0.4, 0.9, 0.5),
                    custom_size: Some(MONSTER_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            },
            collider: Collider::cuboid(MONSTER_SIZE.x/2., MONSTER_SIZE.y/2.),
            body: RigidBody::Dynamic,
            constraints: LockedAxes::ROTATION_LOCKED,
            events: ActiveEvents::CONTACT_FORCE_EVENTS,
            enemy: Enemy,
            monster: Monster
        }
    }
}

pub fn insert_monster_at(commands: &mut Commands, x: usize, y: usize/* , materials: &Res<Materials> */) {
    commands
        .spawn_bundle(MonsterBundle::new(x as f32, y as f32))
        .insert(Name::new("Monster"))
        ;
}
