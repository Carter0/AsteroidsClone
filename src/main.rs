use bevy::prelude::*;

const WINDOWHEIGHT: f32 = 900.0;
const WINDOWWIDTH: f32 = 500.0;

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
        .add_startup_system(add_camera.system())
        .add_startup_system(spawn_player.system())
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// PLAYER CODE
struct Player;

fn spawn_player(mut commands: Commands,
                mut materials: ResMut<Assets<ColorMaterial>>) {

    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert(Player);
}
