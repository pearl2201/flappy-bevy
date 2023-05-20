use std::f64::MIN;
use std::sync::{Arc, Mutex, RwLock};

use bevy::app::CoreSet::Update;
use bevy::transform;
use bevy::{
    prelude::*,
    sprite,
    window::{CursorGrabMode, PresentMode, WindowLevel},
};
use rand::Rng;
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
        .init_resource::<Game>()
        .add_system(sprite_movement)
        .add_system(touch_system)
        .add_system(mouse_click_system)
        .add_system(animate_sprite)
        .add_system(bird_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component, PartialEq, Eq)]
enum ObjectTag {
    Pipe,
    Background,
    Bird,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component, Debug)]
struct Bird {
    speed: f32,
    acc: f32,
    accRotation: f32,
    gohell: bool,
}
#[derive(Default)]
struct PipePart {
    entity: Option<Entity>,
}
#[derive(Default)]
struct Pipe {
    upper: PipePart,
    below: PipePart,
}
#[derive(Resource, Default)]
struct Game {
    pipes: Vec<Pipe>,
    state: i32,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

const GROUND_HEIGHT: f32 = 121.0;
const WIDTH_BACKGROUND: f32 = 736.0;
const HALF_WIDTH_BACKGROUND: f32 = 736.0 / 2.0;
const HEIGHT_SCREEN: f32 = 576.0;
const DISTANCE_BETWEEN_UP_DOWN_PIPES: f32 = 100.0;
const HEIGHT_PIPE: f32 = 319.0;
const DISTANCE_X_BETWEEN_PIPE: f32 = 200.0;
const MIN_SCREEN: f32 = -552.0;
const BIRTH_HEIGHT: f32 = 26.0;
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..4 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("data/background.png"),
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: Vec3::new((x as f32 - 1.0) * HALF_WIDTH_BACKGROUND, 0.0, 0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Direction::Up,
            ObjectTag::Background,
        ));
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("image/ground.png"),
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: Vec3::new(
                        (x as f32 - 1.0) * HALF_WIDTH_BACKGROUND,
                        HEIGHT_SCREEN / 2.0 - GROUND_HEIGHT / 2.0,
                        10.0,
                    ),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: Vec3::new(1.0, -1.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Direction::Up,
            ObjectTag::Background,
        ));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("image/ground.png"),
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: Vec3::new(
                        (x as f32 - 1.0) * HALF_WIDTH_BACKGROUND,
                        -(HEIGHT_SCREEN / 2.0 - GROUND_HEIGHT / 2.0),
                        10.0,
                    ),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Direction::Up,
            ObjectTag::Background,
        ));
    }

    for x in 0..6 {
        let top_below_pipe = rand::thread_rng().gen_range(-36..36);

        let y_below_pipe = top_below_pipe as f32 - HEIGHT_PIPE / 2.0;
        let y_above_pipe =
            top_below_pipe as f32 + DISTANCE_BETWEEN_UP_DOWN_PIPES + HEIGHT_PIPE / 2.0;
        let bellow = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("image/pipe2.png"),
                    transform: Transform {
                        // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                        // This is used to determine the order of our sprites
                        translation: Vec3::new(
                            (x as f32 + 1.0) * DISTANCE_X_BETWEEN_PIPE,
                            y_below_pipe,
                            0.5,
                        ),
                        // The z-scale of 2D objects must always be 1.0,
                        // or their ordering will be affected in surprising ways.
                        // See https://github.com/bevyengine/bevy/issues/4149
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Direction::Up,
                ObjectTag::Pipe,
            ))
            .id();

        let upper = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("image/pipe2.png"),
                    transform: Transform {
                        // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                        // This is used to determine the order of our sprites
                        translation: Vec3::new(
                            (x as f32 + 1.0) * DISTANCE_X_BETWEEN_PIPE,
                            y_above_pipe,
                            0.5,
                        ),
                        // The z-scale of 2D objects must always be 1.0,
                        // or their ordering will be affected in surprising ways.
                        // See https://github.com/bevyengine/bevy/issues/4149
                        scale: Vec3::new(1.0, -1.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Direction::Up,
                ObjectTag::Pipe,
            ))
            .id();
        game.pipes.push(Pipe {
            below: PipePart {
                entity: Some(bellow),
            },
            upper: PipePart {
                entity: Some(upper),
            },
        });
    }

    // commands.spawn(PipeCouple {
    //     below: Pipe {
    //         sprite_bundle: SpriteBundle {
    //             texture: asset_server.load("image/pipe2.png"),
    //             transform: Transform {
    //                 // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
    //                 // This is used to determine the order of our sprites
    //                 translation: Vec3::new((x as f32 + 1.0) * 200.0, y_below_pipe, 0.5),
    //                 // The z-scale of 2D objects must always be 1.0,
    //                 // or their ordering will be affected in surprising ways.
    //                 // See https://github.com/bevyengine/bevy/issues/4149
    //                 scale: Vec3::new(1.0, 1.0, 1.0),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         direction: Direction::Up,
    //         object_tag: ObjectTag::Pipe,
    //     },
    //     upper: Pipe {
    //         sprite_bundle: SpriteBundle {
    //             texture: asset_server.load("image/pipe2.png"),
    //             transform: Transform {
    //                 // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
    //                 // This is used to determine the order of our sprites
    //                 translation: Vec3::new((x as f32 + 1.0) * 200.0, y_above_pipe, 0.5),
    //                 // The z-scale of 2D objects must always be 1.0,
    //                 // or their ordering will be affected in surprising ways.
    //                 // See https://github.com/bevyengine/bevy/issues/4149
    //                 scale: Vec3::new(1.0, -1.0, 1.0),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         direction: Direction::Up,
    //         object_tag: ObjectTag::Pipe,
    //     },
    // });

    let texture_handle = asset_server.load("image/bird.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(36.0, 26.0), 1, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Bird {
            speed: 200.0,
            acc: -5.0,
            accRotation: -60.0,
            gohell: false,
        },
        ObjectTag::Bird,
    ));
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut ObjectTag, &mut Transform)>,
    mut game: ResMut<Game>,
) {
    if game.state == 0 {
        let mut max_background_x = -1000.0;
        let mut max_pipe_x: f32 = -1000.0;

        for (object_tag, mut position) in &mut sprite_position {
            if *object_tag.as_ref() == ObjectTag::Bird {
                continue;
            }
            position.translation.x -= 150.0 * time.delta_seconds();
            if position.translation.x > max_background_x
                && *object_tag.as_ref() == ObjectTag::Background
            {
                max_background_x = position.translation.x
            }
            if position.translation.x > max_pipe_x && *object_tag.as_ref() == ObjectTag::Pipe {
                max_pipe_x = position.translation.x
            }
        }

        for (object_tag, mut position) in &mut sprite_position {
            if position.translation.x < MIN_SCREEN && *object_tag.as_ref() == ObjectTag::Background
            {
                position.translation.x = max_background_x + HALF_WIDTH_BACKGROUND;
            }
        }

        for pipe in game.pipes.iter_mut() {
            if let (Some(upper_entity), Some(below_entity)) = (pipe.upper.entity, pipe.below.entity)
            {
                if let Ok(mut upper) = sprite_position.get_mut(upper_entity) {
                    if upper.1.translation.x < MIN_SCREEN {
                        let top_below_pipe = rand::thread_rng().gen_range(-36..36);

                        let y_below_pipe = top_below_pipe as f32 - HEIGHT_PIPE / 2.0;
                        let y_above_pipe = top_below_pipe as f32
                            + DISTANCE_BETWEEN_UP_DOWN_PIPES
                            + HEIGHT_PIPE / 2.0;

                        upper.1.translation.x = max_pipe_x + DISTANCE_X_BETWEEN_PIPE;
                        upper.1.translation.y = y_above_pipe;
                        if let Ok(mut below) = sprite_position.get_mut(below_entity) {
                            below.1.translation.x = max_pipe_x + DISTANCE_X_BETWEEN_PIPE;
                            below.1.translation.y = y_below_pipe;
                        }
                    }
                }

                // let mut upper =
                //     match sprite_position.get_mut(upper_entity)
                //     {
                //         Ok(x) => x.1,
                //         _ => continue
                //     };
                //     let mut below =
                //     match sprite_position.get_mut(below_entity)
                //     {
                //         Ok(x) => x.1,
                //         _ => continue
                //     };

                //     {
                //     if upper.translation.x < MIN_SCREEN {
                //         upper.translation.x =  max_background_x + HALF_WIDTH_BACKGROUND;
                //         below.translation.x =  max_background_x + HALF_WIDTH_BACKGROUND;
                //     }
                // }
            }
        }
    }
}

fn bird_movement(
    time: Res<Time>,
    mut transforms: Query<(&mut Bird, &mut Transform), With<Bird>>,
    mut game: ResMut<Game>,
) {
    let (mut bird, mut transform) = transforms.single_mut();
    if game.state == 0 && !bird.gohell {
        bird.speed += bird.acc;
        transform.translation.y += bird.speed * time.delta_seconds();
        transform.rotate_z(f32::to_radians(bird.accRotation * time.delta_seconds()));
        if transform.rotation.z <= f32::to_radians(-90.0) {
            transform.rotation.z = f32::to_radians(-90.0);
        }
        if transform.translation.y + BIRTH_HEIGHT / 2.0 >= HEIGHT_SCREEN / 2.0 - GROUND_HEIGHT {
            transform.translation.y = HEIGHT_SCREEN / 2.0 - GROUND_HEIGHT - BIRTH_HEIGHT / 2.0;
            bird.speed = -3.0;
            transform.rotate_z(f32::to_radians(60.0));
        }

        if transform.translation.y - BIRTH_HEIGHT / 2.0 <= -HEIGHT_SCREEN / 2.0 + GROUND_HEIGHT {
            bird.gohell = true;
            game.state = 1;
        }
        //println!("bird transform: {:?}, {:?}",bird, transform.rotation.to_euler(EulerRot::XYZ));
    }
}

fn touch_system(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_released() {
        info!(
            "just released touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_cancelled() {
        info!("cancelled touch with id: {:?}", touch.id());
    }

    // you can also iterate all current touches and retrieve their state like this:
    for touch in touches.iter() {
        info!("active touch: {:?}", touch);
        info!("  just_pressed: {}", touches.just_pressed(touch.id()));
    }
}

fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>, mut transforms: Query<(&mut Bird, &mut Transform), With<Bird>>, mut game: ResMut<Game>,) {
    let (mut bird, mut transform) = transforms.single_mut();    
    if game.state == 0 && !bird.gohell {

        if mouse_button_input.pressed(MouseButton::Left) {
           bird.speed = 200.0;
           transform.rotation =  Quat::from_rotation_z(f32::to_radians(60.0));
           println!("bird click: {:?}",f32::to_radians(60.0));
        }
    }
}
