use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct MainMenuPlugin;

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

#[derive(Component, Debug)]
enum MenuButton {
    Play,
    Quit,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_colors).with_system(button_press))
            ;
    }
}

///
///
///
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // materials: Res<MenuMaterials>,
) {
    let camera_entity = commands.spawn_bundle(Camera2dBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root())
        .insert(Name::new("Root"))
        .with_children(|parent| {
            parent
                .spawn_bundle(border())
                .insert(Name::new("Border"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background())
                        .insert(Name::new("background"))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button())
                                .insert(Name::new("Btn-Play"))
                                .with_children(|btn| {
                                    btn.spawn_bundle(button_text(&asset_server, "New game"));
                                })
                                .insert(MenuButton::Play);
                            parent
                                .spawn_bundle(button())
                                .insert(Name::new("Btn-Quit"))
                                .with_children(|btn| {
                                    btn.spawn_bundle(button_text(&asset_server, "Quit"));
                                })
                                .insert(MenuButton::Quit);
                        });
                });
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
}

///
///
///
fn cleanup(mut commands: Commands, menu_data: Option<Res<MainMenuData>>) {
    if let Some(data) = menu_data {
        commands.entity(data.ui_root).despawn_recursive();
        commands.entity(data.camera_entity).despawn_recursive();
        commands.remove_resource::<MainMenuData>();
    }
}

///
fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    }
}

///
///
///
fn button_text(asset_server: &Res<AssetServer>, label: &str) -> TextBundle {
    return TextBundle::from_section(
        label,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
}

///
///
///
fn root(/*materials: &Res<MenuMaterials>*/) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

///
///
///
fn border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: UiRect::all(Val::Px(8.0)),
            ..Default::default()
        },
        // material: materials.border.clone(),
        ..Default::default()
    }
}

///
///
///
fn menu_background() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}

///
///
///
// fn button_system(
//     materials: Res<MenuMaterials>,
//     mut buttons: Query<
//         (&Interaction, &mut Handle<ColorMaterial>),
//         (Changed<Interaction>, With<Button>),
//     >
// ) {
//     for (interaction, mut material) in buttons.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => *material = materials.button_pressed.clone(),
//             Interaction::Hovered => *material = materials.button_hovered.clone(),
//             Interaction::None => *material = materials.button.clone(),
//         }
//     }
// }

///
///
///
fn button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

///
///
///
fn button_press(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                match button {
                    MenuButton::Play => state
                        .set(AppState::InGame)
                        .expect("Couldn't switch state to InGame"),
                    MenuButton::Quit => exit.send(AppExit),
                };
            }
            _ => {}
        }
    }
}
