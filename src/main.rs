use bevy::{prelude::*, render::camera::ScalingMode};

// Player component
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup)
        .add_startup_stage("player_setup", SystemStage::single(spawn_player)) // line to add
        .run();
}

fn setup(mut commands: Commands) {
    // commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(new_camera_2d());
}

pub fn new_camera_2d() -> Camera2dBundle {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(30.0);
    camera
}

// spawn player system
fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            //material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}
