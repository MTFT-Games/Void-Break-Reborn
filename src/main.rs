use bevy::prelude::*;

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
    ));
}

// Not really needed for anything yet but might be useful later
#[derive(Component)]
struct Background;
