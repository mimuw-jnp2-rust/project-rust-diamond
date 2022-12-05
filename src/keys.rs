use bevy::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const SILVER_KEY_IDX: usize = 12;
pub const GOLD_KEY_IDX: usize = 13;

pub struct KeysPlugin;

#[derive(Component)]
pub struct SilverKeyDetect;

#[derive(Component)]
pub struct GoldKeyDetect;

impl Plugin for KeysPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_keys);
    }
}

// #[derive(Inspectable)]
// enum KeyColour {
//     Silver,
//     Gold,
// }

// #[derive(Component, Inspectable)]
// pub struct Key {
//     colour: KeyColour,
// }

// Creates keys vector and places keys on the map, basing on /assets/map.txt file
fn spawn_keys(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut keys_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let mut idx = 0;
                if char == 's' {
                    idx = SILVER_KEY_IDX;
                } else if char == 'g' {
                    idx = GOLD_KEY_IDX;
                }

                if idx > 0 {
                    let key = spawn_from_textures(
                        &mut commands,
                        &texture,
                        idx,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 160.0),
                    );

                    // commands
                    //     .entity(key)
                    //     .insert(Name::new("Key"))
                    //     .insert(Key {
                    //         colour: KeyColour::Silver,
                    //     })
                    //     .id();
                    
                    if char == 's' {
                        commands.entity(key).insert(SilverKeyDetect);
                    } else {
                        commands.entity(key).insert(GoldKeyDetect);
                    }
                    
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
