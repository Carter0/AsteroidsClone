use crate::logic::player::Player;
use crate::logic::blocks::Block;
use crate::graphics::score::Score;
use crate::logic::spawning::SpawnInfo;

use bevy::prelude::*;


pub struct ResetGamePlugin;

impl Plugin for ResetGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(reset_game.system());
    }
}

fn reset_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    block_query: Query<Entity, With<Block>>,
    mut commands: Commands,
    mut score_query: Query<&mut Score>,
    mut spawn_positions_query: Query<&mut SpawnInfo>,
) {
    if keyboard_input.pressed(KeyCode::R) {

        // Reset player position to 0,0
        if let Ok(mut transform) = player_query.single_mut() {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }

        // Despawn all of the blocks
        for entity in block_query.iter() {
            commands.entity(entity).despawn();
        }

        // Reset the score
        if let Ok(mut score) = score_query.single_mut() {
            // NOTE It adds one automatically in score system
            score.value = -1;
        }

        // Reset the spawn positions
        for mut spawn_positions in spawn_positions_query.iter_mut() {
            spawn_positions.spawned = false;
        }

        // TODO spawn the starting blocks again
    }

}
