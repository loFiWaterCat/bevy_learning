use bevy::{prelude::*, utils::HashMap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_cam)
        .add_startup_system(spawn_player)
        .add_system(animate_sprite)
        .add_system(move_player)
        .run();
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
        Vec2::splat(32.),
        11,
        1,
        None,
        None,
    );

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        Player,
        SpriteAnimation {
            len: 11,
            frame_time: 1. / 20.,
        },
        FrameTime(0.0),
    ));
}

#[derive(Component)]
struct SpriteAnimation {
    len: usize,
    frame_time: f32,
}

#[derive(Component)]
struct FrameTime(f32);

fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }
            frame_time.0 -= animation.frame_time * frames as f32;
        }
    }
}

const MOVE_SPEED: f32 = 100.;

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = player.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVE_SPEED * time.delta_seconds();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Animation {
    MaskRun,
    MaskIdle,
    MaskJump,
    MaskDoubleJump,
    MaskFall,
    Strawberry,
    NinjaRun,
    NinjaIdle,
    NinjaJump,
    NinjaDoubleJump,
    NinjaFall,
    PinkRun,
    PinkIdle,
    PinkJump,
    PinkDoubleJump,
    PinkFall,
    GuyRun,
    GuyIdle,
    GuyJump,
    GuyDoubleJump,
    GuyFall,
}

#[derive(Resource)]
struct Animations {
    map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl FromWorld for Animations {
    fn from_world(world: &mut World) -> Self {
        let mut map = Animations {
            map: HashMap::new(),
        };
        let asset_server = world.resource::<AssetServer>();
        map.add(Animation::MaskIdle)
    }
}

fn change_player_animation(
    mut player: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut SpriteAnimation,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();
    if input.any_just_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        // Set walk animation
    }
    if input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
        && !input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
    {
        // Set idle animation
    }
}
