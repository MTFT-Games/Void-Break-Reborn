use bevy::prelude::*;
use bevy::render::view::RenderLayers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // TODO: Look through defaults and disable things I don't need.
        .add_systems(Startup, spawn_core)
        .add_systems(Update, player_controller)
        .run();
}

/// Spawn the core components needed for basic game function: Background, Player, and Camera
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

/// Core controls for the player
// Todo: Make it all delta time based
fn player_controller(
    mut transform: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    // If there are ever more than one player, something has gone very wrong
    let mut player_transform = transform.single_mut();

    if keyboard.pressed(KeyCode::W) {
        player_transform.translation += Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
    }
    if keyboard.pressed(KeyCode::A) {
        player_transform.translation += Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if keyboard.pressed(KeyCode::S) {
        player_transform.translation += Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };
    }
    if keyboard.pressed(KeyCode::D) {
        player_transform.translation += Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }
}
