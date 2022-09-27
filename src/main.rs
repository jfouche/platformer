use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{prelude::*};

mod components;
mod maps;
mod player;
mod monster;

use components::*;
use maps::spawn_floor;
use monster::death_by_enemy;
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
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup)
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor))
        .add_system(death_by_enemy)
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
