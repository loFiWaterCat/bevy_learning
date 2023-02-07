use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_cam)
        .add_startup_system(spawn_player)
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
        1,
        11,
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
    ));
}

#[derive(Component)]
struct SpriteAnimation {
    len: usize,
    frame_time: f32,
}

#[derive(Component)]
struct FrameTime(f32);

fn animate_sprite(mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if (frame_time.0 > animation.frame_time) {
            let frames = (frame_time.0 / animation.frame_time) as usize;
        }
    }
}
