use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const LIFE_IDX: usize = 5;

pub struct LivesPlugin;

#[derive(Component)]
pub struct LifeDetect;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_lives);
    }
}

// Creates lives vector and places lives on the map, basing on /assets/map.txt file
fn spawn_lives(mut commands: Commands, texture: Res<CharacterTextures>) {
    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut lives_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '+' {
                    let life = spawn_from_textures(
                        &mut commands,
                        &texture,
                        LIFE_IDX,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 150.0),
                    );

                    commands.entity(life).insert(LifeDetect);
                    lives_storage.push(life);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Lives"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&lives_storage);
}
