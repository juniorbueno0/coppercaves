use bevy::{platform::collections::HashSet, prelude::*};
use bevy_northstar::{grid::GridSettingsBuilder, CardinalGrid};

use crate::grid::{DesiredChunks, LoadedChunks, WORLDMAXSIZE};

// #[derive(Debug, Resource)]
// struct DesiredPathChunk(HashSet<(i32, i32)>);

#[derive(Debug, Resource)]
struct LoadedPathChunk(HashSet<(i32, i32)>);

pub struct GridPath;

impl Plugin for GridPath {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedPathChunk(HashSet::new()));

        app.add_systems(Startup, setup_grid);
    }
}

// grid setting 
fn setup_grid(
    mut commands: Commands,
) {
    let grid_settings = GridSettingsBuilder::new_2d(WORLDMAXSIZE as u32, WORLDMAXSIZE as u32).chunk_size(12).build();
    commands.spawn(CardinalGrid::new(&grid_settings));
}

// add grid collisions
