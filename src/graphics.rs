use bevy::prelude::*;

pub struct GraphicsPlugin;

pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>,
    pub player_left: [usize; 5],
    pub player_right: [usize; 5],
    pub player_hit_right: [usize; 2],
    pub player_hit_left: [usize; 2],
    pub player_dead: [usize; 1],
    pub player_hammer_right: [usize; 2],
    pub player_hammer_left: [usize; 2],
}

#[derive(PartialEq, Eq)]
pub enum AnimationDirection {
    Left,
    Right,
    Dead,
    HitRight,
    HitLeft,
    HammerLeft,
    HammerRight,
}

pub enum FacingDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct PlayerGraphics {
    pub animation: AnimationDirection,
    pub facing: FacingDirection,
    pub hammer_done: i32,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub frame_timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub instant_frame: bool,
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
            player_hit_left: [26, 6],
            player_hit_right: [24, 0],
            player_hammer_left: [13, 6],
            player_hammer_right: [12, 0],
            player_dead: [25],
        });
    }

    fn update_player_graphics(
        mut npc_query: Query<(&PlayerGraphics, &mut FrameAnimation), Changed<PlayerGraphics>>,
        characters: Res<CharacterSheet>,
    ) {
        for (graphics, mut animation) in npc_query.iter_mut() {
            animation.frames = match graphics.animation {
                AnimationDirection::Left => characters.player_left.to_vec(),
                AnimationDirection::Right => characters.player_right.to_vec(),
                AnimationDirection::HitRight => characters.player_hit_right.to_vec(),
                AnimationDirection::HitLeft => characters.player_hit_left.to_vec(),
                AnimationDirection::Dead => characters.player_dead.to_vec(),
                AnimationDirection::HammerRight => characters.player_hammer_right.to_vec(),
                AnimationDirection::HammerLeft => characters.player_hammer_left.to_vec(),
            }
        }
    }

    fn frame_animation(
        mut npc_query: Query<(
            &mut TextureAtlasSprite,
            &mut FrameAnimation,
            &mut PlayerGraphics,
        )>,
        time: Res<Time>,
    ) {
        for (mut texture, mut frame, mut player) in npc_query.iter_mut() {
            frame.frame_timer.tick(time.delta());
            if (player.animation == AnimationDirection::HammerRight
                || player.animation == AnimationDirection::HammerLeft)
                && player.hammer_done < 3
            {
                player.hammer_done += 1
            } else if frame.frame_timer.just_finished() {
                frame.current_frame = (frame.current_frame + 1) % frame.frames.len();
                texture.index = frame.frames[frame.current_frame];
            } else if (player.animation == AnimationDirection::HammerRight
                || player.animation == AnimationDirection::HammerLeft)
                && player.hammer_done == 3
            {
                player.hammer_done = 0;
                let animation_left;
                (player, animation_left) = is_animation_left(player);
                if animation_left {
                    player.animation = AnimationDirection::Left;
                } else {
                    player.animation = AnimationDirection::Right;
                }
            }
        }
    }
}

pub fn is_animation_left(graphics: Mut<PlayerGraphics>) -> (Mut<PlayerGraphics>, bool) {
    let is_animation_left = graphics.animation == AnimationDirection::Left
        || graphics.animation == AnimationDirection::HitLeft
        || graphics.animation == AnimationDirection::HammerLeft;
    (graphics, is_animation_left)
}
