use bevy::prelude::*;
use bevy_northstar::{nav::Nav, prelude::{AgentPos, NextPos, Pathfind}, CardinalGrid};

use crate::{mouse::GridClicked, player::{Object, ObjectSelected, SelectedEntities}, world::TILESIZE};

pub struct Worker;

impl Plugin for Worker {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(Update, (worker_selection, apply_worker_movement, get_worker_new_position, lock_unlock_gridcell));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        AgentPos(UVec3::new(4, 4, 0)),
        Transform::from_xyz(12., 0., 4.),
        Sprite { color: Color::srgb(0.6,0.6,0.92), custom_size: Some(Vec2::new(12.0,12.0)), ..default() }
    ));

    commands.spawn((
        Name::new("minion"),
        AgentPos(UVec3::new(4, 4, 0)),
        Transform::from_xyz(12., 0., 4.),
        Sprite { color: Color::srgb(0.6,0.6,0.92), custom_size: Some(Vec2::new(12.0,12.0)), ..default() }
    ));
}

fn worker_selection(
    grid_position: Res<GridClicked>,
    input: Res<ButtonInput<MouseButton>>, 
    object_selected: Res<ObjectSelected>,
    mut entities: ResMut<SelectedEntities>,
    aentities: Query<(&Transform, Entity), With<AgentPos>>
) {
    if input.just_pressed(MouseButton::Left) {
        if object_selected.object == Object::Action || object_selected.object == Object::None {
            for (tf, entity) in aentities.iter() {
                if Vec3::new(tf.translation.x / TILESIZE as f32, tf.translation.y / TILESIZE as f32, tf.translation.z) == Vec3::new(grid_position.position.x as f32, grid_position.position.y as f32, tf.translation.z) {
                    entities.entities.insert(entity);
                }else if entities.entities.contains(&entity) {
                    entities.entities.remove(&entity);
                }
            }
        }
    }
}

fn update_tile_selected(
    // window: Single<&Window>,
    // mut grid_position: ResMut<GridClicked>,
    // camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera>>,
) {
    // let window = window.into_inner();
    // let (camera, camera_transform, _) = camera.into_inner();

    // let hovered_tile = window
    //     .cursor_position()
    //     .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    //     .map(|cursor_position| {
    //         UVec3::new(
    //             (cursor_position.x / TILESIZE as f32).round() as u32,
    //             (cursor_position.y / TILESIZE as f32).round() as u32,
    //             0,
    //         )
    //     });

    // if let Some(tile_position) = hovered_tile {
    //     grid_position.position = tile_position;
    // };
}

fn get_worker_new_position(
    mut commands: Commands,
    grid_position: Res<GridClicked>,
    input: Res<ButtonInput<MouseButton>>,
    entities_selected: Res<SelectedEntities>,
) {
    if input.just_pressed(MouseButton::Right) {
        for e in &entities_selected.entities {
            commands.entity(*e).insert(Pathfind::new(grid_position.position));
        }
    }
}

fn apply_worker_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AgentPos, &NextPos, &mut Transform)>,
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

fn lock_unlock_gridcell(grid: Single<&mut CardinalGrid>,input: Res<ButtonInput<KeyCode>>,grid_position: Res<GridClicked>,) {
    if input.just_pressed(KeyCode::KeyX) {
        let mut grid = grid.into_inner();

        if let Some(nav) = grid.nav(grid_position.position) {
            if !matches!(nav, Nav::Impassable) {
                grid.set_nav(grid_position.position, Nav::Impassable);
            } else {
                grid.set_nav(grid_position.position, Nav::Passable(1));
            }
        } else { return; }
        grid.build();
    }
}