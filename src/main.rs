use bevy::{prelude::*, render::camera::ScalingMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Diamond Rust".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(create_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
        .add_plugins(DefaultPlugins)
        .run();
}

// Spawns player with background (as its child).
fn spawn_player(mut commands: Commands, texture: Res<CharacterTextures>) {
    let mut player_sprite = TextureAtlasSprite::new(0);
    player_sprite.custom_size = Some(Vec2::splat(1.0));

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player_sprite,
            texture_atlas: texture.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0), // player over everything
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .id();

    // background sprite - full square
    let mut background_tile = TextureAtlasSprite::new(10);
    background_tile.custom_size = Some(Vec2::splat(1.0));

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_tile,
            texture_atlas: texture.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0), // player under everything
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}

// Camera spawner.
fn create_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct CharacterTextures(Handle<TextureAtlas>);

// Texture loading and adjusting padding.
fn load_textures(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("character_2-rm.png"); // TODO: adjust resolution
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(45.0), 6, 3, Vec2::splat(30.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(CharacterTextures(atlas_handle));
}
