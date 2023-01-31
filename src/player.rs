use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bushes::BushCollider;
use crate::diamonds::DiamondDetect;
use crate::doors::DoorDetect;
use crate::enemy::Enemy;
use crate::graphics::is_animation_left;
use crate::graphics::AnimationDirection;
use crate::graphics::CharacterSheet;
use crate::graphics::FacingDirection;
use crate::graphics::FrameAnimation;
use crate::graphics::PlayerGraphics;
use crate::keys::KeyDetect;
use crate::lives::LifeDetect;
use crate::save_point::SavePointDetect;
use crate::worldmap::BritleWallDetector;
use crate::worldmap::WallColider;
use crate::TILE_SIZE;
use serde::*;
use serde_json;
use std::fs;

pub const MINIMUM_MOVE_BREAK: f32 = 0.1;
pub const MINIMUM_SPACE_BREAK: f32 = 1.;
pub const MINIMUM_HAMMER_BREAK: f32 = 0.3;
pub const MINIMUM_LIFE_BREAK: f32 = 2.;

pub const START_TILE_X: f32 = 2.0;
pub const START_TILE_Y: f32 = -2.0;

pub struct PlayerPlugin;

#[derive(Component, Inspectable, Serialize, Deserialize)]
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
    space: i32,
    last_space_movement: f32,
    on_save_point: bool,
    death_mode: bool,
    dead: bool,
    health_lost: f32,
    hammer_used: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(put_init_values_to_file)
            .add_startup_system(spawn_player)
            .add_system(
                camera_follow
                    .after("player_movement")
                    .label("camera_follow"),
            )
            .add_system(player_movement.label("player_movement"))
            .add_system(player_collisions.after("camera_follow"));
    }
}

fn put_init_values_to_file() {
    let data = "{\"speed\":10.0,\"health\":3,\"diamonds\":0,\"keys\":0,\"last_up_move\
                    ment\":-100.0,\"last_down_movement\":-100.0,\"last_right_move\
                    ment\":-100.0,\"last_left_movement\":-100.0,\"unchecked_move\
                    ment\":true,\"space\":0,\"last_space_movement\":-100.0,\"on_sa\
                    ve_point\":false,\"death_mode\":false,\"dead\":false,\"health_lost\":-100.0\
                    ,\"hammer_used\":-100.0}";
    let filename = "serialize";
    fs::write(filename, data).expect("Unable to write file");
}

fn spawn_player(mut commands: Commands, characters: Res<CharacterSheet>) {
    let filename = "serialize";
    let context = fs::read_to_string(filename).expect("File not read.");
    let player: Player = serde_json::from_str(&context).unwrap();

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: characters.player_right[0],
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                START_TILE_X * TILE_SIZE,
                START_TILE_Y * TILE_SIZE,
                900.0,
            ),
            texture_atlas: characters.handle.clone(),
            ..Default::default()
        })
        .insert(FrameAnimation {
            frame_timer: Timer::from_seconds(0.2, true),
            frames: characters.player_right.to_vec(),
            current_frame: 0,
            instant_frame: false,
        })
        .insert(PlayerGraphics {
            animation: AnimationDirection::Right,
            facing: FacingDirection::Right,
            hammer_done: 0,
        })
        .insert(Name::new("Player"))
        .insert(player)
        .id();
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
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut PlayerGraphics,
        Entity,
        &mut TextureAtlasSprite,
    )>,
    wall_query: Query<&Transform, (With<WallColider>, Without<Player>)>,
    door_query: Query<&Transform, (With<DoorDetect>, Without<Player>)>,
    enemy_query_transform: Query<&Transform, (With<Enemy>, Without<Player>)>,
    enemy_query_entity: Query<Entity, (With<Enemy>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    britle_query_transform: Query<&Transform, (With<BritleWallDetector>, Without<Player>)>,
    britle_query_entity: Query<Entity, (With<BritleWallDetector>, Without<Player>)>,
    mut commands: Commands,
    characters: Res<CharacterSheet>,
) {
    let (mut player, mut transform, mut graphics, player_entity, mut texture) =
        player_query.single_mut();
    if keyboard.pressed(KeyCode::Space) {
        if player.last_space_movement + MINIMUM_SPACE_BREAK <= time.seconds_since_startup() as f32 {
            player.last_space_movement = time.seconds_since_startup() as f32;

            commands.entity(player_entity).despawn();
            spawn_player(commands, characters);
            return;
        }
    }

    if !player.dead {
        if keyboard.pressed(KeyCode::X) {
            if player.hammer_used + MINIMUM_HAMMER_BREAK <= time.seconds_since_startup() as f32 {
                player.hammer_used = time.seconds_since_startup() as f32;
                let animation_left;
                (graphics, animation_left) = is_animation_left(graphics);
                if animation_left {
                    graphics.animation = AnimationDirection::HammerLeft;
                    texture.index = characters.player_hammer_left.to_vec()[0];
                } else {
                    graphics.animation = AnimationDirection::HammerRight;
                    texture.index = characters.player_hammer_right.to_vec()[0];
                }

                let hammer_direction;
                hammer_direction = match graphics.facing {
                    FacingDirection::Left => Vec3::new(-1.0 * TILE_SIZE, 0.0, 0.0),
                    FacingDirection::Right => Vec3::new(1.0 * TILE_SIZE, 0.0, 0.0),
                    FacingDirection::Up => Vec3::new(0.0, 1.0 * TILE_SIZE, 0.0),
                    FacingDirection::Down => Vec3::new(0.0, -1.0 * TILE_SIZE, 0.0),
                };

                let position_to_check =
                    round_position(transform.translation.clone() + hammer_direction);

                // britle check
                for iter in britle_query_transform
                    .iter()
                    .zip(britle_query_entity.iter())
                {
                    let (britle_transform, britle_entity) = iter;

                    let britle_translation = round_position(britle_transform.translation.clone());
                    let collision = check_simple_collision(&position_to_check, &britle_translation);

                    if collision {
                        commands.entity(britle_entity).despawn(); // despawning britle if hit by hammer
                    }
                }
            }
        }

        let mut y_delta = 0.0;
        if keyboard.pressed(KeyCode::Up) {
            if player.last_up_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32 {
                graphics.facing = FacingDirection::Up;
                y_delta += TILE_SIZE;
                player.last_up_movement = time.seconds_since_startup() as f32;
            }
        }
        if keyboard.pressed(KeyCode::Down) {
            if player.last_down_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32
            {
                graphics.facing = FacingDirection::Down;
                y_delta -= TILE_SIZE;
                player.last_down_movement = time.seconds_since_startup() as f32;
            }
        }

        let mut x_delta = 0.0;
        if keyboard.pressed(KeyCode::Left) {
            if player.last_left_movement + MINIMUM_MOVE_BREAK <= time.seconds_since_startup() as f32
            {
                graphics.facing = FacingDirection::Left;
                x_delta -= TILE_SIZE;
                let animation_left;
                (graphics, animation_left) = is_animation_left(graphics);
                if !animation_left {
                    graphics.animation = AnimationDirection::Left;
                    texture.index = characters.player_left.to_vec()[0];
                }
                player.last_left_movement = time.seconds_since_startup() as f32;
            }
        }
        if keyboard.pressed(KeyCode::Right) {
            if player.last_right_movement + MINIMUM_MOVE_BREAK
                <= time.seconds_since_startup() as f32
            {
                graphics.facing = FacingDirection::Right;
                x_delta += TILE_SIZE;
                let animation_left;
                (graphics, animation_left) = is_animation_left(graphics);
                if animation_left {
                    graphics.animation = AnimationDirection::Right;
                    texture.index = characters.player_right.to_vec()[0];
                }
                player.last_right_movement = time.seconds_since_startup() as f32;
            }
        }

        if x_delta != 0.0 || y_delta != 0.0 {
            let new_exact_position =
                round_position(transform.translation.clone() + Vec3::new(x_delta, y_delta, 0.0));

            let mut collision = would_collide_with_wall(&new_exact_position, &wall_query);

            if !collision {
                if player.keys == 0 {
                    collision = would_collide_with_door(&new_exact_position, &door_query);
                }

                if !collision {
                    transform.translation =
                        transform.translation + Vec3::new(x_delta, y_delta, 0.0);
                    player.unchecked_movement = true;
                }
            }
        }

        let new_exact_position = round_position(transform.translation.clone());

        for iter in enemy_query_transform.iter().zip(enemy_query_entity.iter()) {
            let (enemy_transform, _) = iter;

            let enemy_translation = round_position(enemy_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &enemy_translation);
            if collision
                && player.health_lost + MINIMUM_LIFE_BREAK <= time.seconds_since_startup() as f32
            {
                player.health_lost = time.seconds_since_startup() as f32;
                player.health -= 1;
                if player.health == 0 {
                    player.dead = true;
                }
            }
        }

        if player.health_lost + MINIMUM_LIFE_BREAK > time.seconds_since_startup() as f32 {
            if graphics.animation == AnimationDirection::Left {
                graphics.animation = AnimationDirection::HitLeft;
            } else if graphics.animation == AnimationDirection::Right {
                graphics.animation = AnimationDirection::HitRight;
            }
        } else if graphics.animation == AnimationDirection::HitLeft {
            graphics.animation = AnimationDirection::Left;
        } else if graphics.animation == AnimationDirection::HitRight {
            graphics.animation = AnimationDirection::Right;
        }
    } else {
        graphics.animation = AnimationDirection::Dead;
    }
}

fn player_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    bush_query_transform: Query<&Transform, (With<BushCollider>, Without<Player>)>,
    bush_query_entity: Query<Entity, (With<BushCollider>, Without<Player>)>,
    key_query_transform: Query<&Transform, (With<KeyDetect>, Without<Player>)>,
    key_query_entity: Query<Entity, (With<KeyDetect>, Without<Player>)>,
    diamond_query_transform: Query<&Transform, (With<DiamondDetect>, Without<Player>)>,
    diamond_query_entity: Query<Entity, (With<DiamondDetect>, Without<Player>)>,
    door_query_transform: Query<&Transform, (With<DoorDetect>, Without<Player>)>,
    door_query_entity: Query<Entity, (With<DoorDetect>, Without<Player>)>,
    save_point_query_transform: Query<&Transform, (With<SavePointDetect>, Without<Player>)>,
    save_point_entity: Query<Entity, (With<SavePointDetect>, Without<Player>)>,
    life_query_transform: Query<&Transform, (With<LifeDetect>, Without<Player>)>,
    life_query_entity: Query<Entity, (With<LifeDetect>, Without<Player>)>,
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

        // save_points destruction
        for iter in save_point_query_transform
            .iter()
            .zip(save_point_entity.iter())
        {
            let (save_point_transform, _) = iter;

            let save_point_translation = round_position(save_point_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &save_point_translation);

            player.on_save_point = collision;
            if collision {
                // Saving game state...
                let filename = "serialize";
                let serialization = serde_json::to_string(player.as_ref());
                fs::write(filename, serialization.unwrap()).expect("Unable to write file");
            }
        }

        // diamond pickup
        for iter in diamond_query_transform
            .iter()
            .zip(diamond_query_entity.iter())
        {
            let (diamond_transform, diamond_entity) = iter;

            let diamond_translation = round_position(diamond_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &diamond_translation);

            if collision {
                commands.entity(diamond_entity).despawn(); // despawning diamond if collision
                player.diamonds += 1;
            }
        }

        // key pickup
        for iter in key_query_transform.iter().zip(key_query_entity.iter()) {
            let (key_transform, key_entity) = iter;

            let key_translation = round_position(key_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &key_translation);

            if collision {
                commands.entity(key_entity).despawn(); // despawning key if collision
                player.keys += 1;
            }
        }

        // life pickup
        for iter in life_query_transform.iter().zip(life_query_entity.iter()) {
            let (life_transform, life_entity) = iter;

            let life_translation = round_position(life_transform.translation.clone());
            let collision = check_simple_collision(&new_exact_position, &life_translation);

            if collision {
                commands.entity(life_entity).despawn(); // despawning life if collision
                player.health += 1;
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
pub fn would_collide_with_wall(
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

// Checks if the player movement would cause the door collision.
pub fn would_collide_with_door(
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

pub fn round_position(mut position: Vec3) -> Vec3 {
    position *= 10.0;
    position[0] = position[0].round();
    position[1] = position[1].round();
    position /= 10.0;
    position
}

pub fn check_simple_collision(position1: &Vec3, position2: &Vec3) -> bool {
    position1[0] == position2[0] && position1[1] == position2[1]
}
