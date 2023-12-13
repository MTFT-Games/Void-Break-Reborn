use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rand::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // TODO: Look through defaults and disable things I don't need.
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, (spawn_core, spawn_asteroids))
        .add_systems(Startup, (setup_fps_counter, setup_ui))
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_systems(Update, player_controller.before(movement))
        .add_systems(Update, movement)
        .add_systems(Update, apply_drag)
        .add_systems(Update, (camera_controller, check_collisions).after(wrap))
        .add_systems(Update, wrap.after(movement))
        .add_systems(Update, tick_lifetime)
        .add_systems(Update, update_player_ui)
        .add_systems(
            Update,
            (cull_bullets, break_asteroids, hurt_player).after(check_collisions),
        )
        .add_systems(Update, bevy::window::close_on_esc)
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
                x: 1024.0, // make this a variable to use elsewhere
                y: 1024.0,
            },
        },
        RenderLayers::layer(1),
    ));
    commands
        .spawn((
            // Main camera
            Camera2dBundle::default(),
            RenderLayers::from_layers(&[0, 1]),
            MainCamera,
        ))
        .with_children(|parent| {
            // This is kinda disgusting, make it a loop later TODO
            // Bottom left
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(-1024.0, -1024.0, 0.0),
                    camera: Camera {
                        order: 1,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Bottom middle
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(0.0, -1024.0, 0.0),
                    camera: Camera {
                        order: 2,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Bottom right
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(1024.0, -1024.0, 0.0),
                    camera: Camera {
                        order: 3,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Top left
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(-1024.0, 1024.0, 0.0),
                    camera: Camera {
                        order: 4,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Top middle
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(0.0, 1024.0, 0.0),
                    camera: Camera {
                        order: 5,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Top right
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(1024.0, 1024.0, 0.0),
                    camera: Camera {
                        order: 6,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Left
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(-1024.0, 0.0, 0.0),
                    camera: Camera {
                        order: 7,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
            // Right
            parent.spawn((
                UiCameraConfig { show_ui: false },
                Camera2dBundle {
                    transform: Transform::from_xyz(1024.0, 0.0, 0.0),
                    camera: Camera {
                        order: 8,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
            ));
        });
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            // TODO: Might want to set sprite size
            texture: assets.load("basic_player_100.png"),
            ..Default::default()
        },
        player: Player,
        velocity: Velocity::default(),
        drag: Drag {
            translational: 1.5,
            rotational: 2.0,
        },
        wrap: Wrappable,
        health: Health {
            health: 100.0,
            max: 100.0,
        },
        affiliation: Affiliation::Friendly,
        collision: CollisionConfig { radius: 65.0 },
        damage: Damage::Basic(50.0),
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
    collision: CollisionConfig,
    damage: Damage,
}

#[derive(Component)]
struct Player;

/// Core controls for the player
// Todo: Make it all delta time based
fn player_controller(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // If there are ever more than one player, something has gone very wrong
    let (mut player_velocity, player_transform) = query.single_mut();
    let forward = player_transform.local_y();

    if keyboard.pressed(KeyCode::W) {
        player_velocity.translation_speed += forward * 1000.0 * time.delta_seconds();
    }
    // TODO: lock reverse behind an upgrade later
    if keyboard.pressed(KeyCode::S) {
        player_velocity.translation_speed -= forward * 1000.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::A) {
        player_velocity.rotation_speed += 2.0 * PI * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player_velocity.rotation_speed -= 2.0 * PI * time.delta_seconds();
    }

    if keyboard.just_pressed(KeyCode::Space) {
        commands.spawn((
            ProjectileBundle {
                affiliation: Affiliation::Friendly,
                collision: CollisionConfig { radius: 13.0 },
                damage: Damage::Basic(5.0),
                life: Lifetime {
                    time: Timer::from_seconds(1.5, TimerMode::Once),
                },
                marker: Projectile,
                velocity: Velocity {
                    translation_speed: player_velocity.translation_speed + forward * 500.0,
                    rotation_speed: 0.0,
                },
                sprite_bundle: SpriteBundle {
                    transform: player_transform.clone(),
                    texture: assets.load("basic_bullet_50.png"),
                    ..Default::default()
                },
            },
            Bullet,
            Wrappable,
        ));
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

#[derive(Component, Default)]
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

#[derive(Component, Default)]
struct Asteroid;

#[derive(Component, Default)]
struct Health {
    // Maybe make enum with hits and hp variants
    health: f32,
    max: f32,
}

#[derive(Component, Default)]
struct CollisionConfig {
    // This could eb an enum maybe for different types of collision boxes maybe. or contain one along with other info
    radius: f32,
}

#[derive(Bundle, Default)]
struct AsteroidBundle {
    collision: CollisionConfig,
    health: Health,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    wrap: Wrappable,
    asteroid: Asteroid,
    affiliation: Affiliation,
    damage: Damage,
}

fn spawn_asteroids(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for _ in 0..4 {
        let size: f32 = rng.gen_range(10.0..100.0);
        let direction = rng.gen_range(0.0..PI * 2.0);
        let speed = rng.gen_range(0.0..3000.0 / size);
        commands.spawn(AsteroidBundle {
            collision: CollisionConfig { radius: size / 2.0 },
            health: Health {
                health: size / 2.0,
                max: size / 2.0,
            },
            sprite_bundle: SpriteBundle {
                texture: assets.load("basic_asteroid_100.png"),
                transform: Transform::from_xyz(
                    rng.gen_range(-1000.0..1000.0),
                    rng.gen_range(-1000.0..1000.0),
                    0.0,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: size, y: size }),
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity {
                translation_speed: Vec3 {
                    x: direction.cos(),
                    y: direction.sin(),
                    z: 0.0,
                } * speed,
                rotation_speed: rng.gen_range(-100.0 / size..100.0 / size),
            },
            damage: Damage::Basic(size / 3.0),
            ..Default::default()
        });
    }
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

#[derive(Component, PartialEq, Default)]
enum Affiliation {
    // should this just be part of collision config?
    Friendly,
    #[default]
    Neutral,
    Hostile,
}

#[derive(Component)]
struct Projectile;

#[derive(Component, Clone)]
enum Damage {
    // should this just be part of collision config?
    Basic(f32),
}
impl Default for Damage {
    fn default() -> Self {
        Damage::Basic(0.0)
    }
}

#[derive(Component)]
struct Lifetime {
    time: Timer, // This could also be replaced with health and applying damage over time...
}

#[derive(Bundle)]
struct ProjectileBundle {
    affiliation: Affiliation,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    // TODO: Remove projectile maybe? I don't think I use it...
    marker: Projectile, // TODO: Standardize calling this property marker or the type name
    damage: Damage,
    life: Lifetime,
    collision: CollisionConfig,
}

fn tick_lifetime(time: Res<Time>, mut lifetimes: Query<&mut Lifetime>) {
    for mut lifetime in lifetimes.iter_mut() {
        lifetime.time.tick(time.delta());
    }
}

#[derive(Component)]
struct Bullet;
// Either have bullet and other timed types be in an enum and handle all of their deaths here in a big match
// Or each projectile type has its own system to handle what happens when it dies
// or whenever something dies to lifetime it just despawns and if i want things to happen on death
// use health and constant damage over time... or doo something else special, maybe events can help
// for now this will work
// also, is there any situation where a lifetime ticking and checking to despawn need to be separate? or should all controllers/death handlers also do the ticking...?
fn cull_bullets(
    query: Query<(Entity, &Lifetime), With<Bullet>>,
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (entity, lifetime) in query.iter() {
        if lifetime.time.finished() {
            commands.entity(entity).despawn();
        }
    }
    for collision in collisions.read() {
        if query.contains(collision.entities[0]) {
            commands.entity(collision.entities[0]).despawn();
        }
        if query.contains(collision.entities[1]) {
            commands.entity(collision.entities[1]).despawn();
        }
    }
}

#[derive(Event)]
struct CollisionEvent {
    entities: [Entity; 2],
    damage: [Option<Damage>; 2],
    // Potentially useful in the future: direction vector with magnitude of overlap, knockback stats, weather it needs to be resolved
}

fn check_collisions(
    mut events: EventWriter<CollisionEvent>,
    query: Query<(
        Entity,
        &CollisionConfig,
        &Transform,
        Option<&Affiliation>,
        Option<&Damage>,
    )>,
) {
    // TODO: this might be easier if affiliations were their own components instead of an enum
    for [entity1, entity2] in query.iter_combinations() {
        // TODO Make this more readable
        // In the case that the entities are of the same affiliation, don't even check
        if entity1.3.is_some() && entity1.3 == entity2.3 {
            continue;
        }
        if entity1
            .2
            .translation
            .xy()
            .distance_squared(entity2.2.translation.xy())
            < (entity1.1.radius + entity2.1.radius).powi(2)
        {
            // Collision detected
            events.send(CollisionEvent {
                entities: [entity1.0, entity2.0],
                damage: [entity1.4.cloned(), entity2.4.cloned()],
            });
            println!(
                "Collision between entity {:?} and entity {:?}.",
                entity1.0, entity2.0
            );
        }
    }
}

fn break_asteroids(
    mut query: Query<(Entity, &mut Health, &Transform), With<Asteroid>>,
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    assets: Res<AssetServer>,
) {
    // TODO rewrite cull_bullets in this way maybe. This is also kinda gross tho
    for collision in collisions.read() {
        for i in 0..=1 {
            if let Ok((entity, mut health, transform)) = query.get_mut(collision.entities[i]) {
                // Asteroid collision
                // TODO collision resolution

                if let Some(damage) = &collision.damage[i.abs_diff(1)] {
                    match damage {
                        Damage::Basic(dmg) => {
                            health.health -= dmg;
                        }
                    }

                    // Only need to check if the asteroid should die if its health changed,
                    // which is presumed to only happen here
                    if health.health <= 0.0 {
                        commands.entity(entity).despawn();

                        // Fragment
                        let size = health.max * 2.0;
                        let max_divisions = (size / 10.0).min(5.0) as i32;
                        let divisions = rng.gen_range(0..max_divisions);
                        let new_size = size / divisions.max(2) as f32;
                        // TODO make this more random and conserve momentum
                        for d in 0..divisions {
                            let direction = rng.gen_range(0.0..PI * 2.0);
                            let speed = rng.gen_range(0.0..3000.0 / new_size);
                            commands.spawn(AsteroidBundle {
                                collision: CollisionConfig {
                                    radius: new_size / 2.0,
                                },
                                damage: Damage::Basic(new_size / 3.0),
                                health: Health {
                                    health: new_size / 2.0,
                                    max: new_size / 2.0,
                                },
                                sprite_bundle: SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::splat(new_size)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(transform.translation),
                                    texture: assets.load("basic_asteroid_100.png"),
                                    ..Default::default()
                                },
                                velocity: Velocity {
                                    translation_speed: Vec3 {
                                        x: direction.cos(),
                                        y: direction.sin(),
                                        z: 0.0,
                                    } * speed,
                                    rotation_speed: rng
                                        .gen_range(-100.0 / new_size..100.0 / new_size),
                                },
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }
}

fn hurt_player(
    mut query: Query<(Entity, &mut Health), With<Player>>,
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        for i in 0..=1 {
            if let Ok((entity, mut health)) = query.get_mut(collision.entities[i]) {
                // Player collision
                // TODO collision resolution

                if let Some(damage) = &collision.damage[i.abs_diff(1)] {
                    match damage {
                        Damage::Basic(dmg) => {
                            health.health -= dmg;
                        }
                    }

                    // Only need to check if the player should die if its health changed,
                    // which is presumed to only happen here.
                    if health.health <= 0.0 {
                        // TODO game over. for now this will prob crash on player death
                        commands.entity(entity).despawn();
                    }
                }
                // Since there should only be one player, this skips checking the other
                // collision entity if the first one is the player
                break;
            }
        }
    }
}

#[derive(Component)]
struct UiHealthBack;
#[derive(Component)]
struct UiHealthFront;

fn setup_ui(mut commands: Commands) {
    // TODO: move fps to this root since i think you can only have one root. Maybe????
    // TODO: make it scale well
    // I think this will just be for player health/shield
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                height: Val::Px(50.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,

                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Health background
            parent
                .spawn((
                    NodeBundle {
                        background_color: BackgroundColor(Color::Rgba {
                            red: 0.3,
                            green: 0.3,
                            blue: 0.3,
                            alpha: 1.0,
                        }),
                        style: Style {
                            height: Val::Px(15.0),
                            width: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    UiHealthBack,
                ))
                .with_children(|parent| {
                    // Health foreground
                    parent.spawn((
                        NodeBundle {
                            background_color: BackgroundColor(Color::Rgba {
                                red: 1.0,
                                green: 0.0,
                                blue: 0.0,
                                alpha: 1.0,
                            }),
                            style: Style {
                                width: Val::Percent(50.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        UiHealthFront,
                    ));
                });
        });
}

fn update_player_ui(
    mut health_back: Query<&mut Style, (With<UiHealthBack>, Without<UiHealthFront>)>,
    mut health_front: Query<&mut Style, (With<UiHealthFront>, Without<UiHealthBack>)>,
    player_stats: Query<&Health, With<Player>>,
) {
    // more than one ui is a yike
    let stats = player_stats.single();
    health_back.single_mut().width = Val::Px(stats.max);
    health_front.single_mut().width = Val::Px(stats.health);
}
