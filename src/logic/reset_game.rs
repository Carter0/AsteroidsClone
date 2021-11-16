use super::blocks::SpawnBlockEvent;
use crate::graphics::score::Score;
use crate::logic::blocks::Block;
use crate::logic::player::Player;
use crate::logic::spawning::SpawnInfo;
use bevy::prelude::*;

pub struct ResetGamePlugin;

impl Plugin for ResetGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ResetGameEvent>()
            .add_system(send_reset_game_event.system())
            .add_system(reset_game.system())
            .add_system(reset_player.system());
    }
}

struct ResetGameEvent;

fn send_reset_game_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut reset_game_event: EventWriter<ResetGameEvent>,
) {
    if keyboard_input.pressed(KeyCode::R) {
        reset_game_event.send(ResetGameEvent)
    }
}

fn reset_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut reset_game_event: EventReader<ResetGameEvent>,
){

    for _event in reset_game_event.iter() {
        // Reset player position to 0,0
        if let Ok(mut transform) = player_query.single_mut() {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        } else {

            // If the player is dead, spawn a new one
            super::player::spawn_player(&mut commands, &mut materials)
        }
    }
}

fn reset_game(
    mut reset_game_event: EventReader<ResetGameEvent>,
    block_query: Query<Entity, With<Block>>,
    mut commands: Commands,
    mut score_query: Query<&mut Score>,
    mut spawn_positions_query: Query<(Entity, &mut SpawnInfo)>,
    mut spawn_event: EventWriter<SpawnBlockEvent>,
) {
    for _event in reset_game_event.iter() {
        // Despawn all of the blocks
        for entity in block_query.iter() {
            commands.entity(entity).despawn();
        }

        // Reset the score
        if let Ok(mut score) = score_query.single_mut() {
            // NOTE It adds one automatically in score system
            score.active = true;
            score.value = -1;
        }

        // Reset the spawn positions
        let mut counter = 0;
        for (entity, mut spawn_positions) in spawn_positions_query.iter_mut() {
            spawn_positions.spawned = false;

            if counter < 6 {
                spawn_event.send(SpawnBlockEvent(entity));
                counter += 1;
            }
        }
    }
}
