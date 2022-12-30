use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const DIAMOND_IDX: usize = 8;

pub struct DiamondsPlugin;

#[derive(Component)]
pub struct DiamondDetect;

impl Plugin for DiamondsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_diamonds);
    }
}

// Creates diamonds vector and places diamonds on the map, basing on /assets/map.txt file
fn spawn_diamonds(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut diamonds_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == 'p' {
                    let diamond = spawn_from_textures(
                        &mut commands,
                        &texture,
                        DIAMOND_IDX,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 150.0),
                    );

                    commands.entity(diamond).insert(DiamondDetect);
                    diamonds_storage.push(diamond);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Diamonds"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&diamonds_storage);
}
