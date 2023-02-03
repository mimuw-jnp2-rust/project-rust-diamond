use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const SAVE_POINT_IDX: usize = 26;

pub struct SavePointPlugin;

#[derive(Component)]
pub struct SavePointDetect;

impl Plugin for SavePointPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_points);
    }
}

// Creates save points vector and places them on the map, basing on /assets/map.txt file
fn spawn_points(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut points_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '*' {
                    let save_point = spawn_from_textures(
                        &mut commands,
                        &texture,
                        SAVE_POINT_IDX,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 150.0),
                    );

                    commands.entity(save_point).insert(SavePointDetect);
                    points_storage.push(save_point);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("points"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&points_storage);
}
