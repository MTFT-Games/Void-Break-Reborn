use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // TODO: Look through defaults and disable things I don't need.
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default()) // TODO make this debug only
        .add_systems(Startup, spawn_core)
        .add_systems(Update, player_controller)
        .add_systems(Update, movement)
        .add_systems(Update, apply_drag)
        .add_systems(Update, camera_controller)
        .add_systems(Update, wrap)
        .run();
}

/// Spawn the core components needed for basic game function: Background, Player, and Camera
fn spawn_core(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        // Background
        SpriteBundle {
            texture: assets.load("purple_nebula_4_repeated.png"),
            ..Default::default()
        },
        Background {
            size: Vec2 {
                x: 1024.0,
                y: 1024.0,
            },
        },
        RenderLayers::layer(1),
    ));
    commands.spawn((
        // Main camera
        Camera2dBundle::default(),
        RenderLayers::from_layers(&[0, 1]),
        MainCamera,
    ));
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            // TODO: Might want to set sprite size
            texture: assets.load("basic_player_100.png"),
            ..Default::default()
        },
        player: Player,
        velocity: Velocity::default(),
        drag: Drag {
            translational: 0.5,
            rotational: 2.0,
        },
        wrap: Wrappable,
    });
}

// Not really needed for anything yet but might be useful later
#[derive(Component)]
struct Background {
    size: Vec2,
}

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    player: Player,
    velocity: Velocity,
    drag: Drag,
    wrap: Wrappable,
}

#[derive(Component)]
struct Player;

/// Core controls for the player
// Todo: Make it all delta time based
fn player_controller(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // If there are ever more than one player, something has gone very wrong
    let (mut player_velocity, player_transform) = query.single_mut();
    let forward = player_transform.local_y();

    if keyboard.pressed(KeyCode::W) {
        player_velocity.translation_speed += forward * 200.0 * time.delta_seconds();
    }
    // TODO: lock reverse behind an upgrade later
    if keyboard.pressed(KeyCode::S) {
        player_velocity.translation_speed -= forward * 200.0 * time.delta_seconds();
    }

    use std::f32::consts::PI;
    if keyboard.pressed(KeyCode::A) {
        player_velocity.rotation_speed += 2.0 * PI * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player_velocity.rotation_speed -= 2.0 * PI * time.delta_seconds();
    }
}

#[derive(Component, Default)]
struct Velocity {
    translation_speed: Vec3,
    /// Note: Positive is clockwise
    rotation_speed: f32,
}

fn movement(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.translation_speed * time.delta_seconds();
        transform.rotate_local_z(velocity.rotation_speed * time.delta_seconds());
    }
}

#[derive(Component)]
/// 0-1 or more due to delta time, more drag slows faster
struct Drag {
    translational: f32,
    rotational: f32,
}

fn apply_drag(mut query: Query<(&mut Velocity, &Drag)>, time: Res<Time>) {
    for (mut velocity, drag) in query.iter_mut() {
        // I think this is the right way to delta time here... could be wrong
        velocity.translation_speed *= 1.0 - drag.translational * time.delta_seconds();
        velocity.rotation_speed *= 1.0 - drag.rotational * time.delta_seconds();

        // TODO: zero it past a threshold. or maybe not...
    }
}

#[derive(Component)]
struct MainCamera;

fn camera_controller(
    player_transform: Query<&Transform, With<Player>>,
    // This one must explicitly exclude player or Bevy will scream even though no MainCamera has a Player.
    mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    // Again, if theres more than one player, we have a big issue. TODO: If multiplayer later this will be an issue...
    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation = player_transform.translation;
}

#[derive(Component)]
struct Wrappable;

fn wrap(background: Query<&Background>, mut transforms: Query<&mut Transform, With<Wrappable>>) {
    let background = background.single();
    for mut transform in transforms.iter_mut() {
        if transform.translation.x.abs() > background.size.x / 2.0 {
            transform.translation.x *= -1.0;
        }
        if transform.translation.y.abs() > background.size.y / 2.0 {
            transform.translation.y *= -1.0;
        }
    }
}

#[derive(Component)]
struct Asteroid;

#[derive(Component)]
struct Health {
    health: f32,
}
