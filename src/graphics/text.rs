// TEXT CODE

use bevy::prelude::*;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(game_over_text.system());
    }
}


fn game_over_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_section = TextSection {
        value: "Press R to reset the game. Press ESC to quit.".to_string(),
        style: TextStyle {
            font: asset_server.load("fonts/Roboto-Thin.ttf"),
            font_size: 60.0,
            color: Color::BLACK,
        },
    };

    let text = Text {
        sections: vec![text_section],
        ..Default::default()
    };

    commands
        .spawn_bundle(TextBundle {
            text,
            ..Default::default()
        });
}
