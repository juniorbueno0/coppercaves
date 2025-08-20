use std::collections::HashSet;
use bevy_northstar::{grid::GridSettingsBuilder, nav::Nav, prelude::AgentPos, CardinalGrid};
use noise::{NoiseFn, Perlin};
use bevy::prelude::*;

use crate::camera::MainCameraActualPosition;

#[derive(Resource)]
pub struct ImpassableCoords { coords: HashSet<(i32, i32)> }

#[derive(Debug, Resource)]
struct PerlinInstance { value: Perlin }

#[derive(Resource)]
pub struct LoadedChunks(pub HashSet<(i32, i32)>);

#[derive(Resource)]
pub struct DesiredChunks(pub HashSet<(i32,i32)>);

#[derive(Component)]
pub struct ChunkMarker { pub chunk_coords: (i32, i32) }



pub struct MyGridPlugin;

impl Plugin for MyGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedChunks(HashSet::new()));
        app.insert_resource(DesiredChunks(HashSet::new()));
        app.insert_resource(ImpassableCoords{ coords: HashSet::new() });
        app.insert_resource(PerlinInstance{value:Perlin::new(9)});
    
        app.add_systems(Startup, (setup_grid, set_grid_data.after(setup_grid)));
        app.add_systems(Update, (generate_new_chunk_data, spawn_new_chunks, delete_old_chunks));
    }
}

const CHUNK_SIZE: i32 = 8;
const RENDER_DISTANCE: i32 = 2;
pub const WORLDMAXSIZE: i32 = 50; // defines the max distance in all axis neg and pos
pub const GRIDSQUARESIZE: u32 = 12;

// setup grid
fn setup_grid( mut commands: Commands ) {
    let grid_settings = GridSettingsBuilder::new_2d(WORLDMAXSIZE as u32, WORLDMAXSIZE as u32).chunk_size(12).build();
    commands.spawn(CardinalGrid::new(&grid_settings));   
}

fn set_grid_data(
    mut commands: Commands,
    perlin:Res<PerlinInstance>,
    grid: Single<&mut CardinalGrid>, 
    mut implasable_squares: ResMut<ImpassableCoords>
) {
    let mut grid = grid.into_inner();

    commands.spawn((
        Name::new("Player"),
        AgentPos(UVec3::new(4, 4, 0)),
        Transform::from_xyz(1., 1., 1.),
    ));

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let noise_value: f64 = perlin.value.get([x as f64, y as f64]);
            println!("{:?}", noise_value);
            if generate_impasable_coords(noise_value) == Option::Some(true) && !implasable_squares.coords.contains(&(x as i32, y as i32)) { 
                println!("{:?}", (x as i32, y as i32)); 
                implasable_squares.coords.insert((x as i32, y as i32));
                grid.set_nav(UVec3::new(x as u32, y as u32, 0), Nav::Impassable);
            }
        }
    }

    grid.build();
}

fn generate_new_chunk_data(mut desired_chunks:ResMut<DesiredChunks>,cam_main:Res<MainCameraActualPosition>) {
    let camera_chunk_x = (cam_main.0.x / CHUNK_SIZE as f32).floor() as i32;
    let camera_chunk_y = (cam_main.0.y / CHUNK_SIZE as f32).floor() as i32;
    
    desired_chunks.0 = HashSet::new();
    for chunk_xx in (camera_chunk_x - RENDER_DISTANCE)..=(camera_chunk_x + RENDER_DISTANCE) {
        for chunk_yy in (camera_chunk_y - RENDER_DISTANCE)..=(camera_chunk_y + RENDER_DISTANCE) {
            desired_chunks.0.insert((chunk_xx, chunk_yy));
        }
    }
}

fn delete_old_chunks(mut commands:Commands,query: Query<(Entity, &ChunkMarker), With<ChunkMarker>>,desired_chunks:Res<DesiredChunks>,mut loaded_chunks:ResMut<LoadedChunks>) {
    for (entity, marker) in query.iter() {
        if !desired_chunks.0.contains(&marker.chunk_coords) {
            commands.entity(entity).despawn();
            loaded_chunks.0.remove(&marker.chunk_coords);
        }
    }
}

fn spawn_new_chunks(
    mut commands:Commands,
    perlin:Res<PerlinInstance>,
    desired_chunks:Res<DesiredChunks>,
    mut loaded_chunks:ResMut<LoadedChunks>,
) {
for &chunk_coords in desired_chunks.0.iter() {
        if !loaded_chunks.0.contains(&chunk_coords) {
            let (chunk_x, chunk_y) = chunk_coords;
            // [] WORLD SIZE
            //                  +WORLDMAXSIZE
            //                        | y
            // -WORLDMAXSIZE ---------|---------- +WORLDMAXSIZE
            //               x        |
            //                  -WORLDMAXSIZE
            //
            // from 0 to WORLDMAXSIZE in x and y;

            if (chunk_x > WORLDMAXSIZE || chunk_y > WORLDMAXSIZE) || (chunk_x < -WORLDMAXSIZE || chunk_y < -WORLDMAXSIZE) { continue; }
            
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let world_x: f32 = (chunk_x * CHUNK_SIZE + x) as f32;
                    let world_y: f32 = (chunk_y * CHUNK_SIZE + y) as f32;

                    let noise_x: f64 = (chunk_x * CHUNK_SIZE + x) as f64 / 200.0;
                    let noise_y: f64 = (chunk_y * CHUNK_SIZE + y) as f64 / 200.0;

                    let noise_value: f64 = perlin.value.get([noise_x as f64, noise_y as f64]);

                    commands.spawn((
                        Transform {
                            translation: Vec3::new(world_x * 12.,world_y * 12.,0.),
                            scale: Vec3::new(1. * 12.,1. * 12.,1.),
                            ..default()
                        },
                        Sprite {
                            color: assign_color(noise_value),
                            custom_size: Some(Vec2::new(1.,1.)),
                            ..default()
                        },
                        ChunkMarker { chunk_coords },
                    ));
                }
            }
            loaded_chunks.0.insert(chunk_coords);
        }
    }
}

fn generate_impasable_coords(noise_value: f64) -> Option<bool> {
    match noise_value {
        (-0.8..=-0.5) => { Option::Some(true) },
        (-0.5..=-0.1) => { Option::Some(true) },
        _ => { Option::None }
    }
}

fn assign_color(value: f64) -> Color {
    if (-1.2..=-0.8).contains(&value) {
        Color::srgb(0.0, 0.0, 0.5)
    } else if (-0.8..=-0.5).contains(&value) {
        Color::srgb(0.0, 0.2, 0.8)
    } else if (-0.5..=-0.1).contains(&value) {
        Color::srgb(0.3, 0.5, 1.0)
    } else if (-0.1..=0.0).contains(&value) {
        Color::srgb(1.0, 0.9, 0.6)
    } else if (0.0..=0.4).contains(&value) {
        Color::srgb(0.56, 0.83, 0.43)
    } else if (0.4..=0.8).contains(&value) {
        Color::srgb(0.4, 0.65, 0.28)
    } else if (0.8..=1.2).contains(&value) {
        Color::srgb(0.8, 0.8, 0.8)
    } else {
        Color::srgb(1.0, 1.0, 1.0)
    }
}

// fn lerp(a: f32, b: f32, t: f32) -> f32 {
//     a + (b - a) * t
// }

// fn lerp_color(a: (f32, f32, f32), b: (f32, f32, f32), t: f32) -> Color {
//     Color::srgb(
//         lerp(a.0, b.0, t),
//         lerp(a.1, b.1, t),
//         lerp(a.2, b.2, t),
//     )
// }

// fn pastel_gradient_palette(value: f64) -> Color {
//     // Clamp value to [0.0, 1.0]
//     let v = value.clamp(0.0, 1.);

//     let stops = [
//         (0.0,  (0.77, 0.93, 0.78)), // Light green
//         (0.25, (0.90, 0.97, 0.92)), // Mint
//         (0.5,  (0.98, 0.98, 0.98)), // White
//         (0.75, (0.56, 0.62, 0.73)), // Deep gray blue
//         (1.0,  (0.98, 0.81, 0.62)), // Pastel orange
//     ];

//     for i in 0..(stops.len() - 1) {
//         let (start_val, start_col) = stops[i];
//         let (end_val, end_col) = stops[i + 1];
//         if v >= start_val && v <= end_val {
//             let t = (v - start_val) / (end_val - start_val);
//             return lerp_color(start_col, end_col, t as f32);
//         }
//     }

//     // Fallback (should never hit if v in [0,1])
//     Color::srgb(0.88, 0.86, 0.98)
// }