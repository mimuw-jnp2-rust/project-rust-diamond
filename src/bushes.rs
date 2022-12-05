use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;
use crate::TILE_SIZE;

pub const BUSH_IDX: usize = 3;

pub struct BushesPlugin;

#[derive(Component)]
pub struct BushCollider;

// impl Plugin for BushesPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_startup_system(spawn_bushes);
//         app.add_startup_system(bushes_action);
//     }
// }

impl Plugin for BushesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_bushes);
           // .add_system(bushes_destruction);
    }
}

// Creates bushes vector and places bushes on the map, basing on /assets/map.txt file
fn spawn_bushes(mut commands: Commands, texture: Res<CharacterTextures>) {

    let file = File::open("assets/map.txt").expect("Couldn't open map asset!");
    let mut bushes_storage = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {

                if char == 'b' {
                    let bush = spawn_from_textures(
                        &mut commands,
                        &texture,
                        BUSH_IDX,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 150.0),
                    );

                    commands.entity(bush).insert(BushCollider);
                    bushes_storage.push(bush);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Bushes"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&bushes_storage);
}

// fn bushes_destruction(
//     mut commands: Commands,
//     mut bush_query: Query<Entity, With<BushCollider>>,
//     //keyboard: Res<Input<KeyCode>>,
// ) {
//     for entity in bush_query.iter() {
//     }
//     // if keyboard.pressed(KeyCode::Up) {
//     //     println!("A pressed.");
//     //     destroy_bushes(commands, query);
//     // }
// }

// pub fn destroy_bushes(
//     mut commands: Commands,
//     mut query: Query<Entity, With<BushCollider>>,
// ) {
//     println!("A pressed. I'm here.");
//     for entity in query.iter() {
//         println!("A pressed. I'm here inside.");
//         commands.entity(entity).despawn();
//     }
// }
