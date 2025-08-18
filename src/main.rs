use bevy::{image::ImageSamplerDescriptor, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::WindowResolution};
use bevy_northstar::prelude::*;

mod grid;
mod mouse;
mod camera;
mod worker;
mod grid_path;

fn main() {
    let mut app: App = App::new();

    app.add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings{backends:Some(Backends::VULKAN),..default()}),
        ..default()
    }).set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() }
    ).set(WindowPlugin {
        primary_window: Some(Window {
            resolution:WindowResolution::new(800.,600.).with_scale_factor_override(1.),
            ..default()
        }),
        ..default()
    }));
    
    // pathfinding
    app.add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default());
    app.add_plugins(NorthstarDebugPlugin::<CardinalNeighborhood>::default());
    
    app.add_plugins(mouse::MyMousePlugin);
    app.add_plugins(camera::MyCameraPlugin);

    app.add_plugins(grid::MyGridPlugin);
    app.add_plugins(worker::Worker);

    app.add_plugins(grid_path::GridPath);

    app.run();
}


// limit the world to a specific maz size;
// the grid has to be generated at the start;
// later use the same rules to generate the other grid (the rendered one) hoping this helps with performance!