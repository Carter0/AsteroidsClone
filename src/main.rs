use bevy::app::AppExit;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

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
        .add_plugin(AudioPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(render_background.system())
        .add_startup_system(start_background_audio.system())
        // .add_startup_system(play_music.system())
        // Turn on to see framerate, also import line above
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(logic::spawning::SpawningPlugin)
        .add_plugin(logic::player::PlayerPlugin)
        .add_plugin(logic::blocks::BlocksPlugin)
        .add_plugin(logic::reset_game::ResetGamePlugin)
        .add_plugin(graphics::score::ScorePlugin)
        .add_plugin(graphics::text::TextPlugin)
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn render_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let background_image: Handle<Texture> = asset_server.load("textures/bg.png");

    // Width of standard image in pixels is 272
    let background_width = WINDOWWIDTH / 272.0;
    // Width of standard image in pixels is 160
    let background_height = WINDOWHEIGHT / 160.0;

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(background_image.into()),
        transform: Transform::from_scale(Vec3::new(background_width, background_height, 0.0)),

        ..Default::default()
    });
}

// This is called by the system
#[allow(dead_code)]
fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("sounds/bg_music.mp3"));
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Collidable;
