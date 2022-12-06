use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bushes::BushCollider;
use crate::doors::DoorDetect;
use crate::keys::KeyDetect;
use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::worldmap::WallColider;
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
    keys: usize,
    last_up_movement: f32,
    last_down_movement: f32,
    last_right_movement: f32,
    last_left_movement: f32,
    unchecked_movement: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(
                camera_follow
                    .after("player_movement")
                    .label("camera_follow"),
            )
            .add_system(player_movement.label("player_movement"))
            .add_system(player_interractions.after("camera_follow"));
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
            keys: 0,
            last_up_movement: STARTUP_LAST_MOVEMENT,
            last_down_movement: STARTUP_LAST_MOVEMENT,
            last_right_movement: STARTUP_LAST_MOVEMENT,
            last_left_movement: STARTUP_LAST_MOVEMENT,
            unchecked_movement: true,
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
    wall_query: Query<&Transform, (With<WallColider>, Without<Player>)>,
    door_query: Query<&Transform, (With<DoorDetect>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

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

    if x_delta != 0.0 || y_delta != 0.0 {
        let new_exact_position =
            round_position(transform.translation.clone() + Vec3::new(x_delta, y_delta, 0.0));

        let mut collision = check_wall_collision(&new_exact_position, &wall_query);

        if !collision {
            if player.keys == 0 {
                collision = check_door_collision(&new_exact_position, &door_query);
            }

            if !collision {
                transform.translation = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
                player.unchecked_movement = true;
            }
        }
    }
}

fn player_interractions(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    bush_query_transform: Query<&Transform, (With<BushCollider>, Without<Player>)>,
    bush_query_entity: Query<Entity, (With<BushCollider>, Without<Player>)>,
    key_query_transform: Query<&Transform, (With<KeyDetect>, Without<Player>)>,
    key_query_entity: Query<Entity, (With<KeyDetect>, Without<Player>)>,
    door_query_transform: Query<&Transform, (With<DoorDetect>, Without<Player>)>,
    door_query_entity: Query<Entity, (With<DoorDetect>, Without<Player>)>,
) {
    let (mut player, transform) = player_query.single_mut();
    if player.unchecked_movement {
        let new_exact_position = round_position(transform.translation.clone());

        // bushes destruction
        for iter in bush_query_transform.iter().zip(bush_query_entity.iter()) {
            let (bush_transform, bush_entity) = iter;

            let bush_translation = round_position(bush_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &bush_translation);

            if collision {
                commands.entity(bush_entity).despawn(); // despawning bush if collision
            }
        }

        // key pickup
        for iter in key_query_transform.iter().zip(key_query_entity.iter()) {
            let (key_transform, key_entity) = iter;

            let key_translation = round_position(key_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &key_translation);

            if collision {
                commands.entity(key_entity).despawn(); // despawning bush if collision
                player.keys += 1;
            }
        }

        // door check
        for iter in door_query_transform.iter().zip(door_query_entity.iter()) {
            let (door_transform, door_entity) = iter;

            let door_translation = round_position(door_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &door_translation);

            if collision {
                commands.entity(door_entity).despawn(); // despawning bush if collision
                player.keys -= 1;
            }
        }

        player.unchecked_movement = false;
    }
}

// Checks if the player movement would cause the wall collision.
fn check_wall_collision(
    new_exact_position: &Vec3,
    wall_query: &Query<&Transform, (With<WallColider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let wall_translation = round_position(wall_transform.translation.clone());
        if check_simple_collision(&wall_translation, &new_exact_position) {
            return true; // collision detected
        }
    }

    // no collision
    false
}

// Checks if the player movement would cause the wall collision.
fn check_door_collision(
    new_exact_position: &Vec3,
    door_query: &Query<&Transform, (With<DoorDetect>, Without<Player>)>,
) -> bool {
    for wall_transform in door_query.iter() {
        let wall_translation = round_position(wall_transform.translation.clone());
        if check_simple_collision(&wall_translation, &new_exact_position) {
            return true; // collision detected
        }
    }

    // no collision
    false
}

fn round_position(mut position: Vec3) -> Vec3 {
    position *= 10.0;
    position[0] = position[0].round();
    position[1] = position[1].round();
    position /= 10.0;
    position
}

fn check_simple_collision(position1: &Vec3, position2: &Vec3) -> bool {
    position1[0] == position2[0] && position1[1] == position2[1]
}
