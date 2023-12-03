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
}

// Not really needed for anything yet but might be useful later
#[derive(Component)]
struct Background;
