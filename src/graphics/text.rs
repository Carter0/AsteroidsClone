// TEXT CODE

use bevy::prelude::*;

use crate::logic::player::PlayerDeathEvent;


pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(game_over_text.system());
    }
}

fn game_over_text(
    mut commands: Commands,
    mut player_death_event: EventReader<PlayerDeathEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for _event in player_death_event.iter() {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Press R to reset the game. Press ESC to quit.",
                        TextStyle {
                            font: asset_server.load("fonts/Roboto-thin.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
    }
}

// TODO clear the text if the player hits reset
// fn clear_game_over_text(){}
