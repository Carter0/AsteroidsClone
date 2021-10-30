use crate::{Direction, WINDOWHEIGHT, WINDOWWIDTH};

use bevy::prelude::*;
use rand::Rng;
use std::fmt;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::Startup, spawn_block_positions.system());
    }
}

fn get_direction(integer: i8) -> Direction {
    match integer {
        1 => Direction::Left,
        2 => Direction::Right,
        3 => Direction::Up,
        _ => Direction::Down,
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
        }
    }
}

// TODO
// I feel like the below can be simplified greatly
// by removing spawnlists
//
// I think that the only benefit they provide is indicating whether something is horizontal or
// vertical, which can be contained in the SpawnInfo Struct.

// Contains the locations and directions that
// a block can be spawned in as well as whether
// that position is spawned or not
#[derive(Clone)]
pub struct SpawnInfo {
    pub spawn_location: (i16, i16),
    pub spawn_direction: Direction,
    pub spawned: bool,
    pub direction: BlockDirection,
}

// Contains the spawn locations for all the blocks
// Will be used to actually spawn the blocks later
// pub struct SpawnList {
//     pub horizontal_list: Vec<SpawnInfo>,
//     pub vertical_list: Vec<SpawnInfo>,
// }

impl fmt::Display for SpawnInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Spawn location is ({}, {}), spawn direction is {}, and spawned is {}.",
            self.spawn_location.0, self.spawn_location.1, self.spawn_direction, self.spawned
        )
    }
}

#[derive(Clone, Copy)]
pub enum BlockDirection {
    Horizontal,
    Vertical,
}

// spawning at 500
fn get_edge_of_screen(window_size: f32) -> i16 {
    window_size as i16 / 2
}

fn create_random_blocks<R: Rng>(
    block_positions: Vec<i16>,
    orientation: BlockDirection,
    rng: &mut R,
) -> Vec<SpawnInfo> {
    return block_positions
        .iter()
        .map(|block_position| SpawnInfo {
            spawn_direction: match orientation {
                BlockDirection::Horizontal => get_direction(rng.gen_range(1..=2)),
                BlockDirection::Vertical => get_direction(rng.gen_range(3..=4)),
            },
            spawn_location: match orientation {
                // If you want the blocks to go horizontally (left/right) the y position needs to change
                BlockDirection::Horizontal => (get_edge_of_screen(WINDOWWIDTH), *block_position),
                // If you want the blocks to go vertically (up/down) the x position needs to change
                BlockDirection::Vertical => (*block_position, get_edge_of_screen(WINDOWHEIGHT)),
            },
            spawned: false,
            direction: orientation
        })
        .collect();
}

// Blocks should spawn in set locations depending on
// the size of the screen.
fn create_spawn_locations() -> Vec<SpawnInfo> {
    // Calculate the number of blocks per side
    // ScreenLength / (BlockGap + BlockLength) = BlockNumber
    // Plus 1 because I want one block to spawn at the opposite edge
    let blocks_per_width: i16 = WINDOWWIDTH as i16 / (45 + 80) + 1;
    let blocks_per_height: i16 = WINDOWHEIGHT as i16 / (45 + 80) + 1;

    // Calculate the positions of the blocks per side
    // Need to divide by half because (0,0) is the middle of the screen
    // TODO
    // Redo the names to horizontal and vertical positions because
    // the width and height thing is confusing you.
    // There are horizontal and vertical blocks, not positions here
    let block_width_positions: Vec<i16> = (0..=blocks_per_width)
        .map(|x| WINDOWWIDTH as i16 - (x * 90))
        .map(|x| x - get_edge_of_screen(WINDOWWIDTH))
        .collect();

    let block_height_positions: Vec<i16> = (0..=blocks_per_height)
        .map(|y| WINDOWHEIGHT as i16 - (y * 90))
        .map(|y| y - get_edge_of_screen(WINDOWHEIGHT))
        .collect();

    let mut rng = rand::thread_rng();

    // Create the horizontal and vertical blocks
    let random_horizontal_blocks: Vec<SpawnInfo> =
        create_random_blocks(block_width_positions, BlockDirection::Horizontal, &mut rng);

    let random_vertical_blocks: Vec<SpawnInfo> =
        create_random_blocks(block_height_positions, BlockDirection::Vertical, &mut rng);

    // Combine the blocks together
    [random_horizontal_blocks, random_vertical_blocks].concat()

}

// Create a list of spawn block locations and
// add them as a component to Bevy.
//
// NOTE
// Command usage is only applied between stages,
// so anything that wants to access this data needs to do so in a later
// stage.
//
// TODO
// In Bevy 0.6 this can be rewritten using IteratorCommands
fn spawn_block_positions(mut commands: Commands) {
    for spawn_location in create_spawn_locations() {
        commands.spawn().insert(spawn_location);
    }
}
