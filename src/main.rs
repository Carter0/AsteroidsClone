use bevy::{
    asset::AssetServer,
    core::FixedTimestep,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{
        AlignSelf, App, Assets, Color, ColorMaterial, Commands, DefaultPlugins,
        HorizontalAlign, IntoSystem, OrthographicCameraBundle, PositionType, Query,
        Rect, Res, ResMut, Sprite, SpriteBundle, Style, SystemSet, Text, TextAlignment, TextBundle,
        TextSection, TextStyle, Time, Transform, UiCameraBundle, Val, Vec2, VerticalAlign,
        WindowDescriptor
    },
};

use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

mod logic;

const WINDOWHEIGHT: f32 = 900.0;
const WINDOWWIDTH: f32 = 1000.0;

// For BLOCK_SPAWN_TIMESTEP, it's once every two seconds
const BLOCK_SPAWN_TIMESTEP: f64 = 120.0 / 60.0;
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
        .add_plugin(logic::spawning::SpawningPlugin)
        .add_plugin(logic::player::PlayerPlugin)
        .add_system_set(
            SystemSet::new()
                // This prints out "goodbye world" twice every second
                .with_run_criteria(FixedTimestep::step(BLOCK_SPAWN_TIMESTEP))
                .with_system(spawn_runtime_blocks.system()),
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "goodbye world" twice every second
                .with_run_criteria(FixedTimestep::step(SCORE_ACC_TIMESTEP))
                .with_system(score_update_system.system()),
        )
        .add_system(move_blocks.system())
        // Turn on to see framerate, also import line above
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_starting_block(&mut commands, &mut materials);
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

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            _ => Direction::Down,
        }
    }
}

struct Block {
    velocity: f32,
    direction: Direction,
}

fn spawn_starting_block(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut counter = 0;

    let block_number = 20;
    while counter < block_number {
        spawn_block(&mut commands, &mut materials, Color::rgb(1.0, 0.5, 1.0));
        counter += 1;
    }
}

// spawns blocks as a way to make the game harder during runtime
// this will only run every spawn block timestep
fn spawn_runtime_blocks(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    spawn_block(&mut commands, &mut materials, Color::rgb(0.2, 0.5, 1.0))
}

fn spawn_block(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
) {
    let mut rng = thread_rng();
    let rand_direction: Direction = rand::random();
    let x_starting_position = rng.gen_range(0.0..=WINDOWWIDTH);
    let y_starting_position = rng.gen_range(0.0..=WINDOWHEIGHT);
    let sprite_size_x = 80.0;
    let sprite_size_y = 80.0;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(color.into()),
            transform: Transform::from_xyz(x_starting_position, y_starting_position, 1.0),
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert(Block {
            velocity: 300.0,
            direction: rand_direction,
        })
        .insert(Collidable);
}

// move the block by its own velocity
fn move_blocks(mut block_query: Query<(&Block, &mut Transform, &Sprite)>, time: Res<Time>) {
    for (block, mut transform, sprite) in block_query.iter_mut() {
        let block_speed = block.velocity * time.delta_seconds();
        match &block.direction {
            Direction::Left => transform.translation.x -= block_speed,
            Direction::Right => transform.translation.x += block_speed,
            Direction::Up => transform.translation.y += block_speed,
            Direction::Down => transform.translation.y -= block_speed,
        };

        // Wrap the block if they go off screen
        if transform.translation.x > WINDOWWIDTH / 2.0 + sprite.size.x {
            transform.translation.x = -WINDOWWIDTH / 2.0;
        }

        if transform.translation.x < -WINDOWWIDTH / 2.0 - sprite.size.x {
            transform.translation.x = WINDOWWIDTH / 2.0;
        }

        if transform.translation.y > WINDOWHEIGHT / 2.0 + sprite.size.y {
            transform.translation.y = -WINDOWHEIGHT / 2.0;
        }

        if transform.translation.y < -WINDOWHEIGHT / 2.0 - sprite.size.y {
            transform.translation.y = WINDOWHEIGHT / 2.0;
        }
    }
}

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
