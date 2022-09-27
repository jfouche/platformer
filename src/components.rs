use bevy::prelude::*;

// Player component
#[derive(Component)]
pub struct Player {
    pub speed: f32
}

// Jumper component
#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool
}

#[derive(Component)]
pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub floor_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Monster;