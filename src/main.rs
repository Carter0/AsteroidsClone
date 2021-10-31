use bevy::prelude::*;

mod graphics;
mod logic;

const WINDOWHEIGHT: f32 = 1200.0;
const WINDOWWIDTH: f32 = 1500.0;
const BLOCKSIZEX: f32 = 40.0;
const BLOCKSIZEY: f32 = 40.0;

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
        // TODO
        // refactor blocks to use the spawning code
        // This might involve some event of some kind
        .add_plugin(logic::blocks::BlocksPlugin)
        .add_plugin(graphics::score::ScorePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Collidable;
