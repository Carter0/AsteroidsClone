use bevy::{
    asset::AssetServer,
    core::FixedTimestep,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{
        AlignSelf, App, Color, Commands, DefaultPlugins,
        HorizontalAlign, IntoSystem, OrthographicCameraBundle, PositionType, Query,
        Rect, Res, Style, SystemSet, Text, TextAlignment, TextBundle,
        TextSection, TextStyle, UiCameraBundle, Val, VerticalAlign,
        WindowDescriptor
    },
};

mod logic;

const WINDOWHEIGHT: f32 = 900.0;
const WINDOWWIDTH: f32 = 1000.0;


// For SCORE_ACC_TIMESTEP, it's once every two seconds
const SCORE_ACC_TIMESTEP: f64 = 1.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Asteroids Clone".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        // Turn on to see framerate, also import line above
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(logic::spawning::SpawningPlugin)
        .add_plugin(logic::player::PlayerPlugin)
        .add_plugin(logic::blocks::BlocksPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SCORE_ACC_TIMESTEP))
                .with_system(score_update_system.system()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    render_score(&mut commands, &asset_server);
}

// TODO its time to organize this project into modules
// Block Spawning Code

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Collidable;


// Score

struct Score(i32);

fn render_score(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

    // NOTE
    // I need to keep messing around with this until I get it in the top right corner
    let style = Style {
        // TODO what is this doing
        align_self: AlignSelf::FlexEnd,
        // TODO what is this doing
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
            style: style,
            text: text,
            ..Default::default()
        })
        .insert(Score(0));
}

// TODO this function should be called every frame
fn score_update_system(mut score_query: Query<(&mut Score, &mut Text)>) {
    let (mut score, mut text) = score_query
        .single_mut()
        .expect("There should only be one score in the game.");

    // accumulate the score
    score.0 += 1;
    let string_score: String = score.0.to_string();
    text.sections[0].value = string_score;
}
