use bevy::prelude::*;
use bevy_northstar::{nav::Nav, prelude::{AgentPos, NextPos, Pathfind}, CardinalGrid};

use crate::{player::{Object, ObjectSelected, SelectedEntities}, world::TILESIZE};

pub struct Worker;

impl Plugin for Worker {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(Update, (move_player, input, entity_selection));
    }
}

fn setup(mut commands: Commands) {
        commands.spawn((
            Name::new("Player"),
            AgentPos(UVec3::new(4, 4, 0)), // Starting position in the grid.
            Transform::from_xyz(12., 0., 4.),
            Sprite { color: Color::srgb(0.6,0.6,0.92), custom_size: Some(Vec2::new(12.0,12.0)), ..default() }
        ));
}

fn entity_selection(
    window: Single<&Window>,
    input: Res<ButtonInput<MouseButton>>, 
    object_selected: Res<ObjectSelected>,
    mut entities: ResMut<SelectedEntities>,
    aentities: Query<(&Transform, Entity), With<AgentPos>>,
    camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera>>,
) {
    if input.just_pressed(MouseButton::Left) {
        if object_selected.object == Object::Action || object_selected.object == Object::None {
            let window = window.into_inner();
            let (camera, camera_transform, _) = camera.into_inner();

            let clicked_tile = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
                .map(|cursor_position| {
                    UVec3::new(
                        (cursor_position.x / TILESIZE as f32).round() as u32,
                        (cursor_position.y / TILESIZE as f32).round() as u32,
                        0,
                    )
                });

            for (tf, entity) in aentities.iter() {
                if Vec3::new(tf.translation.x / TILESIZE as f32, tf.translation.y / TILESIZE as f32, tf.translation.z) == Vec3::new(clicked_tile.unwrap().x as f32, clicked_tile.unwrap().y as f32, tf.translation.z) {
                    entities.entities.insert(entity);
                    println!("inserted");
                }else if entities.entities.contains(&entity) {
                    entities.entities.remove(&entity);
                    println!("removed");
                }
            }
        }
    }
}

fn move_player(
    mut query: Query<(Entity, &mut AgentPos, &NextPos, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut agent_pos, next_pos, mut transform) in &mut query {
        transform.translation = Vec3::new(
            next_pos.0.x as f32 * 12.0, // Align with the grid cell size.
            next_pos.0.y as f32 * 12.0,
            4.0,
        );

        agent_pos.0 = next_pos.0;
        commands.entity(entity).remove::<NextPos>();
    }
}

fn input(
    input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    entities_selected: Res<SelectedEntities>,
    camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera>>,
    // player: Single<Entity, With<AgentPos>>,
    grid: Single<&mut CardinalGrid>,
    mut commands: Commands,
) {
    // left click
    let window = window.into_inner();
    let (camera, camera_transform, _) = camera.into_inner();
    // let player = player.into_inner();

    let clicked_tile = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        .map(|cursor_position| {
            UVec3::new(
                (cursor_position.x / TILESIZE as f32).round() as u32,
                (cursor_position.y / TILESIZE as f32).round() as u32,
                0,
            )
        });

    if input.just_pressed(MouseButton::Right) {
        if let Some(goal) = clicked_tile {
            for e in &entities_selected.entities {
                commands.entity(*e).insert(Pathfind::new(goal));
            }
        }
    }

    // if input.just_pressed(MouseButton::Right) {
    //     if let Some(position) = clicked_tile {
    //         let mut grid = grid.into_inner();

    //         if let Some(nav) = grid.nav(position) {
    //             if !matches!(nav, Nav::Impassable) {
    //                 grid.set_nav(position, Nav::Impassable);
    //             } else {
    //                 grid.set_nav(position, Nav::Passable(1));
    //             }
    //         } else { return; }
    //         grid.build();
    //     }
    // }
}