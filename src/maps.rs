use std::cmp::max;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::monster::insert_monster_at;

pub fn spawn_floor(mut commands: Commands/* , mut _materials: Res<Materials> */) {
    let world = create_world(150);
    add_sprites(&mut commands/* , _materials */, &world);
    add_colliders(&world, &mut commands);
    add_enemies(&mut commands, &world/*, &materials*/);

}

fn create_world(width: usize) -> Vec<u8> {
    let mut heights: Vec<u8> = Vec::with_capacity(width);
    let mut height = 1u8;
    (0..width).for_each(|_| {
        heights.push(height);
        height = get_next_height(height)
    });
    heights
}

fn add_tile(commands: &mut Commands /* , materials: &Res<Materials> */, x: f32, height: f32) {
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.8, 0.85, 0.85),
            custom_size: Some(Vec2::new(1., height)),
            ..Default::default()
        },
        ..Default::default()
    };

    let pos = Transform::from_xyz(x, height / 2., 0.0);

    commands
        .spawn_bundle(sprite_bundle)
        .insert(Name::new("Tile"))
        .insert_bundle(TransformBundle::from(pos));
}

fn add_colliders(world: &Vec<u8>, commands: &mut Commands) {
    let max = match world.iter().max() {
        Some(m) => m,
        _ => panic!("add_colliders: World is empty"),
    };
    (1..=*max).for_each(|floor_height| {
        let mut start: Option<usize> = None;
        world
            .iter()
            .enumerate()
            .for_each(|(index, height_at_index)| {
                if *height_at_index >= floor_height && start.is_none() {
                    start = Some(index);
                } else if *height_at_index < floor_height && start.is_some() {
                    add_collider(commands, floor_height, start.unwrap_or(0), index);
                    start = None
                }
            });

        if start.is_some() {
            add_collider(commands, floor_height, start.unwrap_or(0), world.len());
        }
    })
}

fn add_collider(commands: &mut Commands, height: u8, from: usize, to: usize) {
    let half_width = (to - from) as f32 / 2.;
    let x = from as f32 + half_width - 0.5;

    let rigid_body = RigidBody::Fixed;
    let collider = Collider::cuboid(half_width, 0.5);
    let pos = Transform::from_xyz(x, height as f32 - 0.5, 0.0);

    commands
        .spawn()
        .insert(Name::new("Tile-Collider"))
        .insert(rigid_body)
        .insert(collider)
        .insert_bundle(TransformBundle::from(pos));
}

fn add_sprites(commands: &mut Commands /* , materials: Res<Materials> */, world: &Vec<u8>) {
    world.iter().enumerate().for_each(|(x, height)| {
        add_tile(commands /* , &materials */, x as f32, *height as f32);
    });
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

// Function to insert monsters
fn add_enemies(commands: &mut Commands, world: &Vec<u8>/* , materials: &Res<Materials> */) {
    world.iter().enumerate().for_each(|(x, height)| {
        if should_add_enemy(x) {
            insert_monster_at(commands, x, (*height + 1).into() /* , materials */)
        }
    })
}

// Determines whether we should add a monster or not
fn should_add_enemy(x: usize) -> bool {
    if x <= 5 {
        return false;
    }
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    match random_number {
        0..=90 => false,
        _ => true,
    }
}