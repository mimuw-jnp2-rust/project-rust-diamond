use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::textures::spawn_from_textures;
use crate::textures::CharacterTextures;

pub const START_TILE_X: f32 = -2.0;
pub const START_TILE_Y: f32 = 2.0;

use crate::TILE_SIZE;

pub struct EquipmentPlugin;

#[derive(Component, Inspectable)]
pub struct Equipment {
    text: String,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_equipment);
    }
}

fn calculate_offsets() -> (f32, f32) {

}

fn spawn_equipment(mut commands: Commands, texture: Res<CharacterTextures>) {
    let equipment = spawn_from_textures(
        &mut commands,
        &texture,
        3,
        Vec3::new(START_TILE_X * TILE_SIZE, START_TILE_Y * TILE_SIZE, 900.0),
    );

    let (off_x, off_y) = calculate_offsets();
    commands
        .entity(equipment)
        .insert(Name::new("Equipment"))
        .insert(Equipment {
            text: "Aaa.".to_string(),
            offset_x: off_x,
            offset_y: off_y,
        })
        .id();
}
