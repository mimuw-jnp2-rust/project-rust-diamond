use bevy::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const DOOR_IDX: usize = 24;
pub const DOOR_CHAR: char = 'd';

pub struct DoorsPlugin;

#[derive(Component)]
pub struct DoorDetect;

impl Plugin for DoorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_doors);
    }
}

// Creates doors vector and places doors on the map, basing on /assets/map.txt file
fn spawn_doors(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut doors_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == DOOR_CHAR {
                    let door = spawn_from_textures(
                        &mut commands,
                        &texture,
                        DOOR_IDX,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 160.0),
                    );
                    
                    commands.entity(door).insert(DoorDetect);
                    
                    doors_storage.push(door);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Doors"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&doors_storage);
}
