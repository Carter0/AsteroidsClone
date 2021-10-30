// BLOCKS CODE

// use crate::logic::spawning::{SpawnInfo};
use crate::{Collidable, Direction, WINDOWHEIGHT, WINDOWWIDTH};

// use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
// use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use super::spawning::SpawnInfo;

// For BLOCK_SPAWN_TIMESTEP, it's once every two seconds
const BLOCK_SPAWN_TIMESTEP: f64 = 120.0 / 60.0;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Needs to be run after spawning logic
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_starting_block.system())
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(FixedTimestep::step(BLOCK_SPAWN_TIMESTEP))
            //         .with_system(spawn_runtime_blocks.system()),
            // )
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
    mut spawn_positions_query: Query<&mut SpawnInfo>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO
    // Nothing spawns because you only have a vec of spawninfo in the world
    // you don't have individual spawn infos
    let block_number = 40;
    for mut init_spawn_positions in spawn_positions_query.iter_mut().take(block_number) {
        spawn_block(
            &mut commands,
            &mut materials,
            &mut init_spawn_positions,
            Color::rgb(1.0, 0.5, 1.0),
        );
    }

    // let mut counter = 0;

    // let block_number = 5;
    // while counter < block_number {
    //     spawn_block(
    //         &mut commands,
    //         &mut materials,
    //         &mut spawn_positions,
    //         Color::rgb(1.0, 0.5, 1.0),
    //     );
    //     counter += 1;
    // }
}

// spawns blocks as a way to make the game harder during runtime
// this will only run every spawn block timestep
// fn spawn_runtime_blocks(
//     mut commands: Commands,
//     mut spawn_positions_query: Query<&mut SpawnList>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let mut spawn_positions = spawn_positions_query
//         .single_mut()
//         .expect("There should only be one instance of spawn positions");

//     spawn_block(
//         &mut commands,
//         &mut materials,
//         &mut spawn_positions,
//         Color::rgb(0.2, 0.5, 1.0),
//     );
// }

// fn get_list_orientation(integer: i8, spawn_list: &mut SpawnList) -> &mut Vec<SpawnInfo> {
//     match integer {
//         1 => &mut spawn_list.horizontal_list,
//         _ => &mut spawn_list.vertical_list,
//     }
// }

// TODO
// this should only be called if there are still blocks left to spawn
fn spawn_block(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_position: &mut SpawnInfo,
    color: Color,
) {
    // let mut rng = thread_rng();

    // Randomly pick a position based on whether its been spawned or not
    // let random_position: &mut SpawnInfo = spawn_positions
    //     .iter_mut()
    //     .filter(|spawn_position| spawn_position.spawned == false)
    //     .choose(&mut rng)
    //     .unwrap();

    // TODO
    // These needs to be consts somewhere outside
    // this function because the spawning code uses it
    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    let location = spawn_position.spawn_location;
    let direction = spawn_position.spawn_direction;

    // set the positions spawned value to true
    spawn_position.spawned = true;

    // TODO
    // Also you never set spawned to true lol
    //
    // TODO
    // I think the block limit then respawn a good idea

    // NOTE
    // honestly I kinda like it without touching the spawn bool

    // TODO
    // Bug with staying in the corner
    //
    //
    // TODO stop score accumulation
    //
    // TODO Respawn button

    println!("x position: {}", location.0);
    println!("y position: {}", location.1);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(location.0 as f32, location.1 as f32, 1.0),
            ..Default::default()
        })
        .insert(Block {
            velocity: 300.0,
            direction,
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
