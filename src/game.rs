use crate::{maps::spawn_floor, components::Materials};

use super::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_floor))
            // .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_map.system()))
            // .add_system_set(SystemSet::on_update(AppState::InGame).with_system(back_to_main_menu_controls.system()))
            .add_startup_system(setup);
    }
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

fn back_to_main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}