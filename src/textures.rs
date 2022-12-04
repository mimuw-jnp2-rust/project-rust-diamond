use bevy::prelude::*;

use crate::TILE_SIZE;

pub const TEXTURES_ROWS: usize = 4;
pub const TEXTURES_COLUMNS: usize = 4;

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_textures);
    }
}

// Spawns sprite with image no. idx from textures.
pub fn spawn_from_textures(
    commands: &mut Commands,
    texture: &CharacterTextures,
    idx: usize,
    translation: Vec3,
) -> Entity {
    assert!(idx < TEXTURES_ROWS * TEXTURES_COLUMNS);

    let mut sprite = TextureAtlasSprite::new(idx);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: texture.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub struct CharacterTextures(Handle<TextureAtlas>);

// Texture loading and adjusting padding.
fn load_textures(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("character16.png"); // TODO: adjust transparency
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(42.0),
        TEXTURES_COLUMNS,
        TEXTURES_ROWS,
        Vec2::splat(7.0),
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(CharacterTextures(atlas_handle));
}
