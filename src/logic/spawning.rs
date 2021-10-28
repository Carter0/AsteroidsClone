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

// Contains the locations and directions that
// a block can be spawned in as well as whether
// that position is spawned or not
#[derive(Clone)]
pub struct SpawnInfo {
    pub spawn_location: (i16, i16),
    pub spawn_direction: Direction,
    pub spawned: bool,
}

// Contains the spawn locations for all the blocks
// Will be used to actually spawn the blocks later
pub struct SpawnList {
    pub horizontal_list: Vec<SpawnInfo>,
    pub vertical_list: Vec<SpawnInfo>,
}

impl fmt::Display for SpawnInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Spawn location is ({}, {}), spawn direction is {}, and spawned is {}.",
            self.spawn_location.0, self.spawn_location.1, self.spawn_direction, self.spawned
        )
    }
}

enum BlockDirection {
    Horizontal,
    Vertical,
}

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
                BlockDirection::Horizontal => (*block_position, get_edge_of_screen(WINDOWWIDTH)),
                BlockDirection::Vertical => (get_edge_of_screen(WINDOWHEIGHT), *block_position),
            },
            spawned: false,
        })
        .collect();
}

// Blocks should spawn in set locations depending on
// the size of the screen.
fn create_spawn_locations() -> SpawnList {
    // Calculate the number of blocks per side
    // ScreenLength / (BlockGap + BlockLength) = BlockNumber
    let blocks_per_width: i16 = WINDOWWIDTH as i16 / (45 + 80);
    let blocks_per_height: i16 = WINDOWHEIGHT as i16 / (45 + 80);

    // Calculate the positions of the blocks per side
    // Need to divide by half because (0,0) is the middle of the screen
    let block_width_positions: Vec<i16> = (1..=blocks_per_width)
        .map(|x| WINDOWWIDTH as i16 - (x * 90))
        .map(|x| x - get_edge_of_screen(WINDOWWIDTH))
        .collect();

    let block_height_positions: Vec<i16> = (1..=blocks_per_height)
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
    SpawnList {
        horizontal_list: random_horizontal_blocks,
        vertical_list: random_vertical_blocks,
    }
}

// Create a list of spawn block locations and
// add them as a component to Bevy.
//
// NOTE
// Command usage is only applied between stages,
// so anything that wants to access this data needs to do so in a later
// stage.
fn spawn_block_positions(mut commands: Commands) {
    commands.spawn().insert(create_spawn_locations());
}
