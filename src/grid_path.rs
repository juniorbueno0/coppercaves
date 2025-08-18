use bevy::{platform::collections::HashSet, prelude::*};

use crate::grid::{DesiredChunks, LoadedChunks};

// #[derive(Debug, Resource)]
// struct DesiredPathChunk(HashSet<(i32, i32)>);

#[derive(Debug, Resource)]
struct LoadedPathChunk(HashSet<(i32, i32)>);

pub struct GridPath;

impl Plugin for GridPath {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedPathChunk(HashSet::new()));

        app.add_systems(Update, generate_path_grid);
    }
}

fn generate_path_grid(
    loaded_chunks: Res<LoadedChunks>,
    mut loaded_pchunks: ResMut<LoadedPathChunk>,
    commands: Commands,
) {
    for coord in loaded_chunks.0.iter() {
        if !loaded_pchunks.0.contains(coord) {
            // let grid = create_pathfinding_grid_for_chunk(coord);
            // commands.spawn(grid).insert(PathGrid { coord, grid });
            // loaded_pchunks.0.insert(coord.clone());
        }
    }
}