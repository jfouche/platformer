use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{prelude::*};

mod components;
mod maps;
mod player;

use components::*;
use maps::spawn_floor;
use player::*;

///
/// 
/// 
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup)
        .add_startup_stage("player_setup", SystemStage::single(spawn_player))
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor))
        .add_system(player_jumps)
        .add_system(jump_reset)
        .add_system(player_movement)
        .run();
}

///
/// 
/// 
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // commands.spawn_bundle(new_camera_2d());
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        floor_material: materials.add(Color::rgb(0.8, 0.85, 0.85).into()),
    });
}

///
/// 
/// 
pub fn new_camera_2d() -> Camera2dBundle {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(30.0);
    camera
}



// fn spawn_floor(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {
//     let width = 10.;
//     let height = 1.;
//     let rigid_body = RigidBody::Fixed;
    
//     // Bundle {
//     //     position: Vec2::new(0.0, -2.).into(),
//     //     body_type: RigidBodyType::Static,
//     //     ..Default::default()
//     // };
//     let collider = Collider::cuboid(width / 2., height / 2.);

//     commands
//         .spawn_bundle(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(0.7, 0.7, 0.7),
//                 custom_size: Some(Vec2::new(width, height)),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(rigid_body)
//         .insert(collider)
//         .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -4.0, 0.0)))
//         ;
//         // .insert(RigidBodyPositionSync::Discrete);
// }

/*
fn add_tile(commands: &mut Commands, materials: &Res<Materials>, x: f32) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x, -2.).into(),
        body_type: RigidBodyType::Static,
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.floor_material.clone(),
            sprite: Sprite::new(Vec2::new(1., 1.)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}
 */
