use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const EMPTY_IDX: usize = 23;

pub const BRITLE_IDX: usize = 22;
pub const OBSTACLE_IDX: usize = 18;
pub const GRASS_IDX: usize = 23;

pub struct WorldMapPlugin;

#[derive(Component)]
pub struct WallColider;

#[derive(Component)]
pub struct BritleWallDetector;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

// Creates map basing on /assets/map.txt file
fn create_map(mut commands: Commands, texture: Res<CharacterTextures>) {
    let tiles_symbols = HashMap::from([
        ('x', OBSTACLE_IDX),
        ('o', GRASS_IDX),
        ('b', GRASS_IDX),
        ('0', BRITLE_IDX),
        ('s', GRASS_IDX),
        ('g', GRASS_IDX),
    ]);

    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut map_tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let mut tile_idx = EMPTY_IDX;

                let find_idx = tiles_symbols.get(&char);
                if find_idx.is_some() {
                    tile_idx = *find_idx.unwrap();
                }

                let tile = spawn_from_textures(
                    &mut commands,
                    &texture,
                    tile_idx,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );

                if char == 'x' || char == '0' {
                    commands.entity(tile).insert(WallColider);
                    if char == '0' {
                        let tile_background = spawn_from_textures(
                            &mut commands,
                            &texture,
                            GRASS_IDX,
                            Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 99.0),
                        );
                        map_tiles.push(tile_background);
                        commands.entity(tile).insert(BritleWallDetector);
                    }
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
