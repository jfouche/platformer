use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{prelude::*};

// Player component
#[derive(Component)]
struct Player;

// Jumper component
#[derive(Component)]
struct Jumper {
    jump_impulse: f32,
    is_jumping: bool
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
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_stage("player_setup", SystemStage::single(spawn_player))
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor))
        .add_system(player_jumps)
        .add_system(jump_reset)
        .run();
}

///
/// 
/// 
fn setup_camera(mut commands: Commands) {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(30.0);
    commands.spawn_bundle(camera);
}

///
///  spawn player system
/// 
fn spawn_player(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {

    let player_size = Vec2::new(1.0, 1.0);
    let rigid_body = RigidBody::Dynamic;
    let collider = Collider::cuboid(player_size.x / 2., player_size.y / 2.);
    // let collider = ColliderBundle {

    // };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(player_size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Jumper { jump_impulse: 30., is_jumping: false})
        .insert(rigid_body)
        .insert(collider)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.4,
        });
}

///
/// 
/// 
fn spawn_floor(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {
    let width = 10.;
    let height = 1.;
    let rigid_body = RigidBody::Fixed;
    
    // Bundle {
    //     position: Vec2::new(0.0, -2.).into(),
    //     body_type: RigidBodyType::Static,
    //     ..Default::default()
    // };
    let collider = Collider::cuboid(width / 2., height / 2.);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(rigid_body)
        .insert(collider)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -4.0, 0.0)))
        ;
        // .insert(RigidBodyPositionSync::Discrete);
}

///
/// 
/// 
fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse);
            jumper.is_jumping = true;
        }
    }
}

///
/// 
/// 
fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

///
/// 
/// 
fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &CollisionEvent) {
    if let CollisionEvent::Started(h1, h2, _) = event {
        if h1 == &entity || h2 == &entity {
            jumper.is_jumping = false
        }
    }
}