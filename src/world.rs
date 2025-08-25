use bevy_northstar::{grid::GridSettingsBuilder, nav::Nav, prelude::DebugGridBuilder, CardinalGrid};
use noise::{NoiseFn, Perlin};
use bevy::{platform::collections::HashSet, prelude::*};

use crate::camera::MainCameraActualPosition;

#[derive(Debug, Component)]
struct GridSquare;

#[derive(Debug, Resource)]
struct PerlinInstance { value: Perlin }

#[derive(Debug, Resource)]
struct ChunkData { values: Vec<((i32,i32), Color)> }

#[derive(Resource)]
pub struct DesiredChunks{ pub chunks: HashSet<(i32,i32)> }

#[derive(Resource)]
pub struct LoadedChunks{ pub chunks: HashSet<(i32, i32)> }

pub struct MyWorldPlugin;

impl Plugin for MyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkData { values: Vec::new() });
        app.insert_resource(LoadedChunks{ chunks: HashSet::new() });
        app.insert_resource(DesiredChunks{ chunks: HashSet::new() });
        app.insert_resource(PerlinInstance{value:Perlin::new(9)});

        app.add_systems(Startup, (generate_path_grid, setup_grid_data.after(generate_path_grid)));
        app.add_systems(Update, (generate_new_chunk_data, spawn_new_chunks, cleanup_distant_chunks));
    }
}

const CHUNK_SIZE: i32 = 4;
const RENDER_DISTANCE: i32 = 1;

const TILESIZE: i32 = 12;
const GRIDSIZE: f32 = 60.0;

const CHUNK_WORLD_SIZE: i32 = CHUNK_SIZE * TILESIZE; // 8 * 12 = 96

fn generate_path_grid(mut commands: Commands,) {
    let grid_settings = GridSettingsBuilder::new_2d(GRIDSIZE as u32, GRIDSIZE as u32).chunk_size(TILESIZE as u32).build();
    commands.spawn(CardinalGrid::new(&grid_settings)).with_child((
        DebugGridBuilder::new(TILESIZE as u32, TILESIZE as u32).enable_cells().build(),
    ));
}

fn setup_grid_data(
    perlin: Res<PerlinInstance>,
    grid: Single<&mut CardinalGrid>,
) {
    let mut grid = grid.into_inner();
    for x in 0..GRIDSIZE as i32 {
        for y in 0..GRIDSIZE as i32 {

            let x_noise = ((x * TILESIZE) as f64) / 300.0;
            let y_noise = ((y * TILESIZE) as f64) / 300.0;
            
            let noise_value: f64 = perlin.value.get([x_noise, y_noise]);

            // match noise_value {
            //     (-1.2..=-0.9) => {grid.set_nav(UVec3::new(x as u32, y as u32, 0), Nav::Impassable);},
            //     (-0.9..=-0.6) => {grid.set_nav(UVec3::new(x as u32, y as u32, 0), Nav::Impassable);},
            //     (-0.6..=-0.4) => { grid.set_nav(UVec3::new(x as u32, y as u32, 0), Nav::Impassable);},
            //     _ => {  },
            // };
        }
    }
    grid.build();
}

fn generate_new_chunk_data(mut desired_chunks:ResMut<DesiredChunks>,cam_main:Res<MainCameraActualPosition>) {
    let camera_chunk_x = (cam_main.0.x / CHUNK_WORLD_SIZE as f32).floor() as i32;
    let camera_chunk_y = (cam_main.0.y / CHUNK_WORLD_SIZE as f32).floor() as i32;
    
    desired_chunks.chunks = HashSet::new();
    for chunk_xx in (camera_chunk_x - RENDER_DISTANCE)..=(camera_chunk_x + RENDER_DISTANCE) {
        for chunk_yy in (camera_chunk_y - RENDER_DISTANCE)..=(camera_chunk_y + RENDER_DISTANCE) {
            desired_chunks.chunks.insert((chunk_xx, chunk_yy));
        }
    }
}

fn spawn_new_chunks(
    mut commands:Commands,
    perlin:Res<PerlinInstance>,
    desired_chunks:Res<DesiredChunks>,
    mut loaded_chunks:ResMut<LoadedChunks>,
) {
    for &chunk_coords in desired_chunks.chunks.iter() {
        if !loaded_chunks.chunks.contains(&chunk_coords) {
            let (chunk_x, chunk_y) = chunk_coords;

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let world_x = (chunk_x * CHUNK_SIZE + x) * TILESIZE;
                    let world_y = (chunk_y * CHUNK_SIZE + y) * TILESIZE;

                    let x_noise = (world_x as f64) / 300.0;
                    let y_noise = (world_y as f64) / 300.0;
                    
                    let noise_value: f64 = perlin.value.get([x_noise, y_noise]);

                    let color = match noise_value {
                        (-1.2..=-0.9) => { Color::srgb(0.0, 0.0, 0.5)  },
                        (-0.9..=-0.6) => { Color::srgb(0.0, 0.2, 0.8) },
                        (-0.6..=-0.4) => { Color::srgb(0.3, 0.5, 1.0) },
                        (-0.4..=-0.1) => { Color::srgb(1.0, 0.9, 0.6) },
                        (-0.1..=-0.0) => { Color::srgb(0.56, 0.83, 0.43) },
                        (-0.0..=0.2) => { Color::srgb(0.4, 0.65, 0.28) },
                        (0.2..=0.4) => { Color::srgb(0.8, 0.8, 0.8) },
                        (0.4..=0.6) => { Color::srgb(0.8, 0.8, 0.8)},
                        (0.6..=0.8) => { Color::srgb(0.8, 0.8, 0.8)},
                        _ => { Color::srgb(0.3, 0.3, 0.3) },
                    };

                    commands.spawn((           
                        Transform::from_xyz(world_x as f32, world_y as f32, 1.0),
                        Sprite {
                            color: color,
                            custom_size: Some(Vec2{x:1. * TILESIZE as f32,y:1. * TILESIZE as f32}),
                            ..Default::default()
                        }, GridSquare
                    ));
                }
            }
            loaded_chunks.chunks.insert(chunk_coords);
        }
    }
}

fn cleanup_distant_chunks(
    mut commands: Commands,
    desired_chunks: Res<DesiredChunks>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    sprites_query: Query<(Entity, &Transform), With<GridSquare>>,
) {
    let chunks_to_remove: Vec<(i32, i32)> = loaded_chunks.chunks
        .difference(&desired_chunks.chunks)
        .cloned()
        .collect();
        
    for chunk_coord in chunks_to_remove {
        // Remove sprites for this chunk
        for (entity, transform) in sprites_query.iter() {
            let chunk_x = (transform.translation.x / CHUNK_WORLD_SIZE as f32).floor() as i32;
            let chunk_y = (transform.translation.y / CHUNK_WORLD_SIZE as f32).floor() as i32;
            
            if (chunk_x, chunk_y) == chunk_coord {
                commands.entity(entity).despawn();
            }
        }
        
        loaded_chunks.chunks.remove(&chunk_coord);
    }
}