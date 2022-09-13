use std::cmp::max;

use super::components::Materials;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};


pub fn spawn_floor(mut commands: Commands, mut _materials: ResMut<Materials>) {
    let mut height = 1;
    for x in 0..150 {
        add_tile(&mut commands, &_materials, x as f32, height as f32);
        height = get_next_height(height);
    }
}

fn add_tile(commands: &mut Commands, _materials: &ResMut<Materials>, x: f32, height: f32) {
    let rigid_body = RigidBody::Fixed;
    let collider = Collider::cuboid(0.5, height / 2.);
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.8, 0.85, 0.85),
            custom_size: Some(Vec2::new(1., height)),
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite_bundle)
        .insert(rigid_body)
        .insert(collider)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(x, -2.0, 0.0)))
        ;
}

fn get_random_height_delta() -> i8 {
    let mut rng = thread_rng();
    match rng.gen_range(0..100) {
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    }
}

fn get_next_height(current_height: u8) -> u8 {
    max(current_height as i8 + get_random_height_delta(), 1) as u8
}