use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // TODO: Look through defaults and disable things I don't need.
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, spawn_core)
        .add_systems(Startup, spawn_asteroids)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
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
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
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
        health: Health { health: 100.0 },
        affiliation: Affiliation::Friendly,
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
    health: Health,
    affiliation: Affiliation,
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

fn wrap(
    background: Query<&Background>,
    mut query: Query<(&mut Transform, &Velocity), With<Wrappable>>,
) {
    let background = background.single();
    for (mut transform, velocity) in query.iter_mut() {
        if transform.translation.x.abs() > background.size.x / 2.0
            && transform.translation.x.is_sign_negative()
                == velocity.translation_speed.x.is_sign_negative()
        {
            transform.translation.x *= -1.0;
        }
        if transform.translation.y.abs() > background.size.y / 2.0
            && transform.translation.y.is_sign_negative()
                == velocity.translation_speed.y.is_sign_negative()
        {
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

#[derive(Component)]
struct CollisionConfig {
    radius: f32,
}

#[derive(Bundle)]
struct AsteroidBundle {
    collision: CollisionConfig,
    health: Health,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    wrap: Wrappable,
    asteroid: Asteroid,
    affiliation: Affiliation,
}

fn spawn_asteroids(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(AsteroidBundle {
        asteroid: Asteroid,
        wrap: Wrappable,
        velocity: Velocity {
            translation_speed: Vec3 {
                x: 200.0,
                y: 100.0,
                z: 0.0,
            },
            rotation_speed: PI,
        },
        collision: CollisionConfig { radius: 50.0 },
        health: Health { health: 20.0 },
        sprite_bundle: SpriteBundle {
            texture: assets.load("basic_asteroid_100.png"),
            ..Default::default()
        },
        affiliation: Affiliation::Neutral,
    });

    commands.spawn(AsteroidBundle {
        asteroid: Asteroid,
        wrap: Wrappable,
        velocity: Velocity {
            translation_speed: Vec3 {
                x: -150.0,
                y: -300.0,
                z: 0.0,
            },
            rotation_speed: PI,
        },
        collision: CollisionConfig { radius: 50.0 },
        health: Health { health: 20.0 },
        sprite_bundle: SpriteBundle {
            texture: assets.load("basic_asteroid_100.png"),
            ..Default::default()
        },
        affiliation: Affiliation::Neutral,
    });
    commands.spawn(AsteroidBundle {
        asteroid: Asteroid,
        wrap: Wrappable,
        velocity: Velocity {
            translation_speed: Vec3 {
                x: 100.0,
                y: -170.0,
                z: 0.0,
            },
            rotation_speed: PI,
        },
        collision: CollisionConfig { radius: 50.0 },
        health: Health { health: 20.0 },
        sprite_bundle: SpriteBundle {
            texture: assets.load("basic_asteroid_100.png"),
            ..Default::default()
        },
        affiliation: Affiliation::Neutral,
    });
}

// ============================================================= Ripped from book fps counter

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(mut q: Query<&mut Visibility, With<FpsRoot>>, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

#[derive(Component)]
enum Affiliation {
    Friendly,
    Neutral,
    Hostile,
}

#[derive(Component)]
struct Projectile;

#[derive(Component)]
enum Damage {
    Basic(f32),
}

#[derive(Component)]
enum Lifetime {
    // Is this a good idea to make an enum? we will find out when i try to do stuff with it lol
    Time(Timer), // This could also be replaced with health and applying damage over time...
    Hits(u32),
    // Should health be under this??? maybe? thats a later problem
}

#[derive(Bundle)]
struct ProjectileBundle {
    affiliation: Affiliation,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    marker: Projectile, // TODO: Standardize calling this property marker or the type name
    damage: Damage,
    life: Lifetime,
}
