use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const EMPTY_IDX: usize = 10;

pub const OBSTACLE_CHAR: char = 'x';

pub struct WorldMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

// Creates basing on /assets/map.txt file
fn create_map(mut commands: Commands, texture: Res<CharacterTextures>) {
    let tiles_symbols = HashMap::from([(OBSTACLE_CHAR, 5), ('o', 2), ('b', 3)]);

    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut map_tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let mut tile_idx = EMPTY_IDX;

                let find_idx = tiles_symbols.get(&char);
                if !find_idx.is_none() {
                    tile_idx = *find_idx.unwrap();
                }

                let tile = spawn_from_textures(
                    &mut commands,
                    &texture,
                    tile_idx,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );

                if char == OBSTACLE_CHAR {
                    commands.entity(tile).insert(TileCollider);
                }

                map_tiles.push(tile);
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&map_tiles);
}