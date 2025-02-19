use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

const GRAVITY: f32 = -800.0;
const JUMP_FORCE: f32 = 400.0;
const MOVE_SPEED: f32 = 200.0;

#[derive(Component)]
struct Player {
    velocity: Vec2,
    is_on_ground: bool,
}

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(apply_gravity)
        .add_system(check_collisions)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    // Player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, 50.0, 0.0),
            ..default()
        },
        Player {
            velocity: Vec2::ZERO,
            is_on_ground: false,
        },
    ));

    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(800.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
        Ground,
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }
        player.velocity.x = direction * MOVE_SPEED;
        transform.translation.x += player.velocity.x * time.delta_seconds();

        // Jumping
        if player.is_on_ground && keyboard_input.just_pressed(KeyCode::Space) {
            player.velocity.y = JUMP_FORCE;
            player.is_on_ground = false;
        }
    }
}

fn apply_gravity(mut query: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    for (mut transform, mut player) in &mut query {
        player.velocity.y += GRAVITY * time.delta_seconds();
        transform.translation.y += player.velocity.y * time.delta_seconds();
    }
}

fn check_collisions(
    mut query: Query<(&mut Transform, &mut Player)>,
    ground_query: Query<&Transform, With<Ground>>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        for ground_transform in &ground_query {
            if let Some(_) = collide(
                transform.translation,
                Vec2::new(64.0, 64.0),
                ground_transform.translation,
                Vec2::new(800.0, 20.0),
            ) {
                transform.translation.y = ground_transform.translation.y + 42.0;
                player.velocity.y = 0.0;
                player.is_on_ground = true;
            }
        }
    }
}

