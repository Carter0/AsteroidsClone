// BLOCKS CODE

// use crate::logic::spawning::{SpawnInfo};
use crate::{Collidable, Direction, BLOCKSIZEX, BLOCKSIZEY, WINDOWHEIGHT, WINDOWWIDTH};

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use super::spawning::SpawnInfo;

// For BLOCK_SPAWN_TIMESTEP, it's once every two seconds
const BLOCK_SPAWN_TIMESTEP: f64 = 120.0 / 60.0;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Needs to be run after spawning logic
            .add_event::<SpawnBlockEvent>()
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_starting_block.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(BLOCK_SPAWN_TIMESTEP))
                    .with_system(spawn_runtime_blocks.system()),
            )
            .add_system(move_blocks.system())
            .add_system(spawn_block.system());
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

pub struct Block {
    velocity: f32,
    direction: Direction,
}

pub struct SpawnBlockEvent(pub Entity);

// Spawns starting blocks for the game
fn spawn_starting_block(
    mut spawn_positions_query: Query<(Entity, With<SpawnInfo>)>,
    mut spawn_event: EventWriter<SpawnBlockEvent>,
) {
    let block_number = 6;
    for (entity, _boolean) in spawn_positions_query.iter_mut().take(block_number) {
        spawn_event.send(SpawnBlockEvent(entity))
    }
}

// spawns blocks as a way to make the game harder during runtime
// this will only run every spawn block timestep
fn spawn_runtime_blocks(
    spawn_positions_query: Query<(Entity, &SpawnInfo)>,
    mut spawn_event: EventWriter<SpawnBlockEvent>,
) {
    let mut rng = thread_rng();

    let spawn_entity: Option<Entity> = spawn_positions_query
        .iter()
        .filter(|(_entity, spawn_position)| !spawn_position.spawned )
        .map(|tuple| tuple.0)
        .choose(&mut rng);

    match spawn_entity {
        Some(entity) => {
            spawn_event.send(SpawnBlockEvent(entity))
        }
        // NOTE
        // You could do the swap around stuff here at some point
        None => println!("empty"),
    }
}


// This is called by an event
fn spawn_block(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_event: EventReader<SpawnBlockEvent>,
    mut spawn_query: Query<&mut SpawnInfo>,
) {
    for event in spawn_event.iter() {

        let entity: Entity = event.0;

        if let Ok(mut spawn_position) = spawn_query.get_mut(entity) {

            let texture_handle = asset_server.load("textures/block_1.png");


            let location = spawn_position.spawn_location;
            let direction = spawn_position.spawn_direction;

            // set the positions spawned value to true
            spawn_position.spawned = true;

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(BLOCKSIZEX, BLOCKSIZEY)),
                    material: materials.add(texture_handle.into()),
                    transform: Transform::from_xyz(location.0 as f32, location.1 as f32, 1.0),
                    ..Default::default()
                })
                .insert(Block {
                    velocity: 300.0,
                    direction,
                })
                .insert(Collidable);
        } else {
            // the entity does not have the components from the query
            println!("not here")
        }
    }
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
