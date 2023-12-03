use bevy::prelude::*;
use bevy::render::view::RenderLayers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // TODO: Look through defaults and disable things I don't need.
        .add_systems(Startup, spawn_core)
        .run();
}

fn spawn_core(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.load("purple_nebula_4_repeated.png"),
            ..Default::default()
        },
        Background,
        RenderLayers::layer(1),
    ));
    commands.spawn((
        Camera2dBundle::default(),
        RenderLayers::from_layers(&[0, 1]),
    ));
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            // TODO: Might want to set sprite size
            texture: assets.load("basic_player_100.png"),
            ..Default::default()
        },
        player: Player,
    });
}

// Not really needed for anything yet but might be useful later
#[derive(Component)]
struct Background;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    player: Player,
}

#[derive(Component)]
struct Player;
