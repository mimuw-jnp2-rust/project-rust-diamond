use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::player::check_simple_collision;
use crate::player::round_position;
use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::worldmap::WallColider;
use crate::TILE_SIZE;

pub const ENEMY_IDX: usize = 29;
pub const MINIMUM_ENEMY_MOVEMENT_BREAK: f32 = 0.2;

pub struct EnemyPlugin;

#[derive(Component, Inspectable)]
pub struct Enemy {
    data: i32,
    last_movement: f32,
    dir: i32,
    x_dir: i32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(enemy_spawn_system)
            .add_system(enemy_movement_system);
    }
}

fn enemy_spawn_system(mut commands: Commands, texture: Res<CharacterTextures>) {
    let enemies = vec![Vec3::new(7., 1., 0.), Vec3::new(28., 11., 1.), Vec3::new(2., 11., 1.), Vec3::new(16., 11., 1.)];

    for v in enemies {
        let x = v.x;
        let y = v.y;
        let enemy = spawn_from_textures(
            &mut commands,
            &texture,
            ENEMY_IDX,
            Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 899.0),
        );

        commands
            .entity(enemy)
            .insert(Enemy {
                data: 2,
                last_movement: 2.,
                dir: 1,
                x_dir: v.z as i32,
            })
            .id();
    }
}

fn enemy_movement_system(
    mut enemy_query: Query<(&mut Enemy, &mut Transform)>,
    time: Res<Time>,
    wall_query: Query<&Transform, (With<WallColider>, Without<Enemy>)>,
) {
    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        if enemy.last_movement + MINIMUM_ENEMY_MOVEMENT_BREAK <= time.seconds_since_startup() as f32
        {
            let mut y_delta = 0.0;
            y_delta += TILE_SIZE * (enemy.dir as f32) * ((enemy.x_dir - 1) as f32);
            let mut x_delta = 0.0;
            x_delta += TILE_SIZE * (enemy.dir as f32) * (enemy.x_dir as f32);
            let new_exact_position =
                round_position(transform.translation.clone() + Vec3::new(x_delta, y_delta, 0.0));
            let collision = would_collide_with_wall(&new_exact_position, &wall_query);
            if collision {
                enemy.dir *= -1;
            } else {
                transform.translation = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
                enemy.last_movement = time.seconds_since_startup() as f32;
            }
        }
    }
}

// Checks if the enemy movement would cause the wall collision.
fn would_collide_with_wall(
    new_exact_position: &Vec3,
    wall_query: &Query<&Transform, (With<WallColider>, Without<Enemy>)>,
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
