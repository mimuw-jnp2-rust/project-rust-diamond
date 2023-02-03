use bevy::prelude::*;
use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::worldmap::WallColider;
use crate::TILE_SIZE;

const STONE_IDX: [&'static usize; 5] = [&13, &14, &15, &16, &3];

pub struct StonesPlugin;

#[derive(Component)]
pub struct StoneSystem;

impl Plugin for StonesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_stones);
    }
}

// Creates stones vector and places stones on the map, basing on /assets/map.txt file
fn spawn_stones(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut rng = rand::thread_rng();
    let mut stones_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '@' {
                    let stone_idx_rnd = rng.gen::<usize>() % 5;
                    let stone = spawn_from_textures(
                        &mut commands,
                        &texture,
                        *STONE_IDX[stone_idx_rnd as usize],
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 150.0),
                    );

                    commands
                        .entity(stone)
                        .insert(StoneSystem)
                        .insert(WallColider)
                        .id();
                    stones_storage.push(stone);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Stones"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&stones_storage);
}
