use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::worldmap::TileCollider;
use crate::TILE_SIZE;

pub const PLAYER_SPEED: f32 = 10.0;

pub const MINIMUM_MOVE_BREAK: f32 = 0.1;
pub const STARTUP_LAST_MOVEMENT: f32 = -1.0;

pub const START_TILE_X: f32 = 2.0;
pub const START_TILE_Y: f32 = -2.0;

pub const INIT_HEALTH: usize = 3;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    health: usize,
    diamonds: usize,
    last_up_movement: f32,
    last_down_movement: f32,
    last_right_movement: f32,
    last_left_movement: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow.after("player_movement"))
            .add_system(player_movement.label("player_movement"));
    }
}

fn spawn_player(mut commands: Commands, texture: Res<CharacterTextures>) {
    let player = spawn_from_textures(
        &mut commands,
        &texture,
        0,
        Vec3::new(START_TILE_X * TILE_SIZE, START_TILE_Y * TILE_SIZE, 900.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_SPEED,
            health: INIT_HEALTH,
            diamonds: 0,
            last_up_movement: STARTUP_LAST_MOVEMENT,
            last_down_movement: STARTUP_LAST_MOVEMENT,
            last_right_movement: STARTUP_LAST_MOVEMENT,
            last_left_movement: STARTUP_LAST_MOVEMENT,
        })
        .id();

    let background = spawn_from_textures(&mut commands, &texture, 0, Vec3::new(0.0, 0.0, -1.0));

    commands
        .entity(background)
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, _) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::Up) {
        if player.last_up_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32 {
            y_delta += TILE_SIZE;
            player.last_up_movement = time.seconds_since_startup() as f32;
        }
    }
    if keyboard.pressed(KeyCode::Down) {
        if player.last_down_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32 {
            y_delta -= TILE_SIZE;
            player.last_down_movement = time.seconds_since_startup() as f32;
        }
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::Left) {
        if player.last_left_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32 {
            x_delta -= TILE_SIZE;
            player.last_left_movement = time.seconds_since_startup() as f32;
        }
    }
    if keyboard.pressed(KeyCode::Right) {
        if player.last_right_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32 {
            x_delta += TILE_SIZE;
            player.last_right_movement = time.seconds_since_startup() as f32;
        }
    }

    move_if_not_collision_with_wall(Vec3::new(x_delta, y_delta, 0.0), player_query, &wall_query);
}

// Checks if the player movement would cause the collision. If not, moves the player.
fn move_if_not_collision_with_wall(
    delta_position: Vec3,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    let (_, mut transform) = player_query.single_mut();
    let new_player_pos = transform.translation + delta_position * 0.1;

    for wall_transform in wall_query.iter() {
        let collision = collide(
            new_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return true; // collision detected
        }
    }
    
    // no collision - moving the player
    transform.translation = transform.translation + delta_position;
    false
}
