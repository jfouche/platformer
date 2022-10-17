use crate::{
    components::{Materials, Player},
    maps::spawn_floor,
};

use super::AppState;
use bevy::{prelude::*, render::camera::ScalingMode};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_camera)
                .with_system(spawn_floor),
        )
        // .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_map))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(back_to_main_menu_controls)
                .with_system(camera_follow_player),
        )
        .add_startup_system(setup);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(new_camera_2d());
}

///
///
///
fn new_camera_2d() -> Camera2dBundle {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(30.0);
    camera
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
fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if *app_state.current() == AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}

///
///
///
fn camera_follow_player(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    if let Ok(player) = set.p1().get_single() {
        let (x, y) = (player.translation.x, player.translation.y);
        if let Ok(mut camera) = set.p0().get_single_mut() {
            camera.translation.x = x;
            camera.translation.y = y;
        }
    }
}
