// SCORE CODE

use bevy::asset::AssetServer;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

// For SCORE_ACC_TIMESTEP, it's once every two seconds
const SCORE_ACC_TIMESTEP: f64 = 1.0;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(render_score.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(SCORE_ACC_TIMESTEP))
                    .with_system(score_update_system.system()),
            );
    }
}

pub struct Score {
    pub value: i32,
    pub active: bool,
}

// NOTE
// I have no clue what a lot of the styling/positions does here.
// Will need to come back to this at some point.
fn render_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_section = TextSection {
        value: 0.to_string(),
        style: TextStyle {
            font: asset_server.load("fonts/Roboto-Thin.ttf"),
            font_size: 60.0,
            color: Color::BLACK,
        },
    };

    let text = Text {
        sections: vec![text_section],
        alignment: TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    };

    let style = Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(60.0),
            right: Val::Px(80.0),

            // default is spawning in the lower left hand corner
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(TextBundle {
            style,
            text,
            ..Default::default()
        })
        .insert(Score{value: 0, active: true});
}

fn score_update_system(mut score_query: Query<(&mut Score, &mut Text)>) {
    let (mut score, mut text) = score_query
        .single_mut()
        .expect("There should only be one score in the game.");

    // accumulate the score if its active
    if score.active {
        score.value += 1;
        let string_score: String = score.value.to_string();
        text.sections[0].value = string_score;
    }
}
