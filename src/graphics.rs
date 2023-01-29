use bevy::prelude::*;

pub struct GraphicsPlugin;

pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>,
    pub player_left: [usize; 5],
    pub player_right: [usize; 5],
    pub player_hit_right: [usize; 2],
    pub player_hit_left: [usize; 2],
    pub player_dead: [usize; 1],
}

#[derive(PartialEq)]
pub enum FacingDirection {
    Left,
    Right,
    Dead,
    HitRight,
    HitLeft
}

#[derive(Component)]
pub struct PlayerGraphics {
    pub facing: FacingDirection,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub last_frame_time: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics)
            .add_system(Self::frame_animation)
            .add_system(Self::update_player_graphics);
    }
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let image = assets.load("characters.png");
        let atlas =
            TextureAtlas::from_grid_with_padding(image, Vec2::splat(48.0), 6, 5, Vec2::splat(4.0));
        let atlas_handle = texture_atlases.add(atlas);

        commands.insert_resource(CharacterSheet {
            handle: atlas_handle,
            player_left: [6, 7, 8, 9, 10],
            player_right: [0, 1, 2, 3, 4],
            player_hit_left: [6, 26],
            player_hit_right: [0, 24],
            player_dead: [25],
        });
    }

    fn update_player_graphics(
        mut npc_query: Query<(&PlayerGraphics, &mut FrameAnimation), Changed<PlayerGraphics>>,
        characters: Res<CharacterSheet>,
    ) {
        for (graphics, mut animation) in npc_query.iter_mut() {
            animation.frames = match graphics.facing {
                FacingDirection::Left => characters.player_left.to_vec(),
                FacingDirection::Right => characters.player_right.to_vec(),
                FacingDirection::HitRight => characters.player_hit_right.to_vec(),
                FacingDirection::HitLeft => characters.player_hit_left.to_vec(),
                FacingDirection::Dead => characters.player_dead.to_vec(),
            }
        }
    }

    fn frame_animation(
        mut npc_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in npc_query.iter_mut() {
            animation.last_frame_time.tick(time.delta());
            if animation.last_frame_time.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}