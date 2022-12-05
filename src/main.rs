use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;
pub const WINDOW_HEIGHT: f32 = 900.0;

mod debug;
use debug::DebugPlugin;

mod player;
use player::PlayerPlugin;

mod worldmap;
use worldmap::WorldMapPlugin;

mod textures;
use textures::TexturesPlugin;

mod bushes;
use bushes::BushesPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_HEIGHT * RESOLUTION,
            height: WINDOW_HEIGHT,
            title: "Diamond Rust".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_camera)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldMapPlugin)
        .add_plugin(BushesPlugin)
        .add_plugin(TexturesPlugin)
        .run();
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
