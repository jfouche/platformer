use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;


mod components;
mod maps;
mod player;
mod monster;
mod game;
mod main_menu;
mod bullets;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use player::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

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
        // debug plugins
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // Game
        .add_state(AppState::MainMenu)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_system(main_menu_controls)
        .add_system(display_events)
        .run();
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

///
/// 
/// 
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

///
/// 
/// 
fn main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::MainMenu {
        if keys.just_pressed(KeyCode::Return) {
            app_state.set(AppState::InGame).unwrap();
            keys.reset(KeyCode::Return);
        }
    } else {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}