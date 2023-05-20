use bevy::{
    prelude::*,
    sprite,
    window::{CursorGrabMode, PresentMode, WindowLevel},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                resolution: (736., 576.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("data/background.png"),
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: Vec3::new(0.0, 0.0, 0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Direction::Up,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("data/background.png"),
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: Vec3::new(-368.0, 0.0, 0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Direction::Up,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("data/background.png"),
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: Vec3::new(368.0, 0.0, 0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Direction::Up,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("data/background.png"),
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: Vec3::new(736.0, 0.0, 0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Direction::Up,
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    let mut max_x = -1000.0;

    for (mut _logo, mut position) in &mut sprite_position {
        position.translation.x -= 150.0 * time.delta_seconds();
        if position.translation.x > max_x {
            max_x = position.translation.x
        }
    }

    for (mut _logo, mut position) in &mut sprite_position {
        if position.translation.x < -552.0 {
            position.translation.x = max_x + 368.0
        }
    }
}
