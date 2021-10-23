// BLOCKS CODE

use crate::{Collidable, Direction, WINDOWHEIGHT, WINDOWWIDTH};

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

// For BLOCK_SPAWN_TIMESTEP, it's once every two seconds
const BLOCK_SPAWN_TIMESTEP: f64 = 120.0 / 60.0;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(spawn_starting_block.system())
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

fn spawn_starting_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
