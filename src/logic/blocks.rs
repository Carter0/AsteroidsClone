// BLOCKS CODE

use crate::logic::spawning::{SpawnInfo, SpawnList};
use crate::{Collidable, CommonLabels, Direction, WINDOWHEIGHT, WINDOWWIDTH};

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

// For BLOCK_SPAWN_TIMESTEP, it's once every two seconds
const BLOCK_SPAWN_TIMESTEP: f64 = 120.0 / 60.0;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TODO this needs to go after the spawning logic
        // So this needs a label
        app.add_startup_system(
            spawn_starting_block
                .system()
                .label(CommonLabels::BlockLogic)
                .after(CommonLabels::Spawning),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(BLOCK_SPAWN_TIMESTEP))
                .with_system(spawn_runtime_blocks.system()),
        )
        .add_system(move_blocks.system());
    }
}

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

// Spawns starting blocks for the game
fn spawn_starting_block(
    mut commands: Commands,
    mut spawn_positions_query: Query<&mut SpawnList>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let spawn_positions = spawn_positions_query.iter_mut();
    // I checked and the spawning code does run??
    // My only running theory right now is that
    // while the spawning code does run first it doesn't finish
    // by the time this system starts running.

    // println!("{}", spawn_positions.len());
    //
    // TODO
    // Here is the fix
    // .add_startup_system_to_stage(StartupStage::PostStartup, spawn_starting_block)

    // NOTE
    // this currently returns nothing :(
    let mut spawn_positions = spawn_positions_query
        .single_mut()
        .expect("There should only be one instance of spawn positions");

    let mut counter = 0;

    let block_number = 20;
    while counter < block_number {
        spawn_block(
            &mut commands,
            &mut materials,
            &mut spawn_positions,
            Color::rgb(1.0, 0.5, 1.0),
        );
        counter += 1;
    }
}

// spawns blocks as a way to make the game harder during runtime
// this will only run every spawn block timestep
fn spawn_runtime_blocks(
    mut commands: Commands,
    mut spawn_positions_query: Query<&mut SpawnList>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spawn_positions = spawn_positions_query
        .single_mut()
        .expect("There should only be one instance of spawn positions");

    spawn_block(
        &mut commands,
        &mut materials,
        &mut spawn_positions,
        Color::rgb(0.2, 0.5, 1.0),
    );
}

// TODO
// Alright look, what do you want???
//
// I think what I want is for the SpawnList to be a component
// that can be used by all the systems here
// Then we just pass in the component to the systems that need it.
// Then all we are doing is updating the component as we go.
//
// How do we add components?
// Fairly simple, components.spawn().insert()
//
//
// Once you do that you can pas
//
// What do you have??
//
//
// What I have...
//
// Contains the locations and directions that
// a block can be spawned in as well as whether
// that position is spawned or not
// #[derive(Clone)]
// struct SpawnInfo {
//     spawn_location: (i16, i16),
//     spawn_direction: Direction,
//     spawned: bool,
// }

// // Contains the spawn locations for all the blocks
// // Will be used to actually spawn the blocks later
// struct SpawnList {
//     spawn_list: Vec<SpawnInfo>,
// }

// TODO
// I think this function should spawn a block based on the spawn positions
fn spawn_block(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_positions: &mut SpawnList,
    color: Color,
) {
    // TODO
    // 1. Filter through the list of spawn positions for unfilled positions
    // 2. Randomly choose a spawn position of the remaining ones
    // 3. Spawn that one

    let mut rng = thread_rng();
    let random_position: &SpawnInfo = spawn_positions
        .spawn_list
        .iter()
        .filter(|spawn_position| spawn_position.spawned == false)
        .choose(&mut rng)
        .unwrap();

    // TODO
    // Do I want to keep around this totally random spawning???
    // Maybe have some kind of combination of both?
    //
    // let mut rng = thread_rng();
    // let rand_direction: Direction = rand::random();
    // let x_starting_position = rng.gen_range(0.0..=WINDOWWIDTH);
    // let y_starting_position = rng.gen_range(0.0..=WINDOWHEIGHT);
    let sprite_size_x = 80.0;
    let sprite_size_y = 80.0;

    // struct SpawnInfo {
    //     spawn_location: (i16, i16),
    //     spawn_direction: Direction,
    //     spawned: bool,
    // }

    let location = random_position.spawn_location;
    let direction = random_position.spawn_direction;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(color.into()),
            transform: Transform::from_xyz(location.0 as f32, location.1 as f32, 1.0),
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert(Block {
            velocity: 300.0,
            direction: direction,
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
