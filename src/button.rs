use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_colors);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub trait ButtonSpawner<'w, 's> {
    fn spawn_button(
        &mut self,
        label: &String,
        asset_server: &Res<AssetServer>,
    ) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's> ButtonSpawner<'w, 's> for ChildBuilder<'w, 's, '_> {
    fn spawn_button(
        &mut self,
        label: &String,
        asset_server: &Res<AssetServer>,
    ) -> EntityCommands<'w, 's, '_> {
        let mut e = self.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
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
        });
        e.with_children(|btn| {
            btn.spawn_bundle(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
        e
    }
}

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
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}
