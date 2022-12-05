use bevy::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const KEY_IDX: usize = 4;
pub const KEY_CHAR: char = 'k';

pub struct KeysPlugin;

#[derive(Component)]
pub struct KeyDetect;

impl Plugin for KeysPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_keys);
    }
}

// Creates keys vector and places keys on the map, basing on /assets/map.txt file
fn spawn_keys(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut keys_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == KEY_CHAR {
                    let key = spawn_from_textures(
                        &mut commands,
                        &texture,
                        KEY_IDX,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 160.0),
                    );
                    
                    commands.entity(key).insert(KeyDetect);
                    
                    keys_storage.push(key);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Keys"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&keys_storage);
}
