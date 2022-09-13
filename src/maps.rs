use super::components::Materials;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn spawn_floor(mut commands: Commands, mut _materials: ResMut<Materials>) {
    for x in 0..50 {
        add_tile(&mut commands, &_materials, x as f32)
    }
}

fn add_tile(commands: &mut Commands, _materials: &ResMut<Materials>, x: f32) {
    let rigid_body = RigidBody::Fixed;
    let collider = Collider::cuboid(0.5, 0.5);
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.8, 0.85, 0.85),
            custom_size: Some(Vec2::new(1., 1.)),
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
