// PLAYER CODE

use crate::graphics::score::Score;
use crate::{Collidable, WINDOWHEIGHT, WINDOWWIDTH};

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_startup_player.system())
            .add_system(move_player.system())
            .add_system(player_collision_system.system());
    }
}

// The float value is the player movement speed in 'pixels/second'.
pub struct Player {
    pub velocity: f32,
    pub teleport_distance: f32,
}

fn spawn_startup_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    spawn_player(&mut commands, &mut materials);
}

pub fn spawn_player(commands: &mut Commands,
                    materials: &mut ResMut<Assets<ColorMaterial>>) {
    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert(Player {
            velocity: 300.0,
            teleport_distance: 70.0,
        })
        .insert(Collidable);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform, &Sprite)>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform, sprite)) = player_query.single_mut() {
        // Get input from the keyboard (WASD)
        let up: bool = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down: bool =
            keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left: bool =
            keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right: bool =
            keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        // If left is pressed than it will be -1, right 1, both they cancel out.
        let x_axis: i8 = -(left as i8) + right as i8;
        let y_axis: i8 = -(down as i8) + up as i8;
        let move_delta: Vec2 = Vec2::new(x_axis as f32, y_axis as f32);

        // move the player
        let delta_time = time.delta_seconds();
        transform.translation.x += move_delta.x * player.velocity * delta_time;
        transform.translation.y += move_delta.y * player.velocity * delta_time;

        // Wrap the player if they go off screen
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

        // teleport the player if they press space
        if keyboard_input.just_pressed(KeyCode::Space) {
            if y_axis == -1 {
                transform.translation.y -= player.teleport_distance;
            }

            if y_axis == 1 {
                transform.translation.y += player.teleport_distance;
            }

            if x_axis == 1 {
                transform.translation.x += player.teleport_distance;
            }

            if x_axis == -1 {
                transform.translation.x -= player.teleport_distance;
            }
        }
    }
}

// simple, player collides with block system
fn player_collision_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Sprite, &Transform), With<Player>>,
    collider_query: Query<&Transform, (With<Collidable>, Without<Player>)>,
    mut score_query: Query<&mut Score>,
) {
    if let Ok((player_entity, sprite, player_transform)) = player_query.single_mut() {
        let player_size = sprite.size;

        for transform in collider_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_size,
                transform.translation,
                sprite.size,
            );

            if let Some(_collision) = collision {
                // Remove the player if they collide with a block
                commands.entity(player_entity).despawn();

                // Stop accumulating the score
                let mut score = score_query
                    .single_mut()
                    .expect("There should only be one score in the game.");

                score.active = false;
            }
        }
    }
}
