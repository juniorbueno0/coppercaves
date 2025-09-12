use bevy::{image::ImageSamplerDescriptor, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::WindowResolution};
use bevy_northstar::prelude::*;

mod grid;
mod mouse;
mod world;
mod player;
mod camera;
mod worker;
mod ui_game;

// pending *create gamestates

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

    app.add_plugins(ui_game::GameUi);
    app.add_plugins(player::Player);

    app.add_plugins(world::MyWorldPlugin);
    app.add_plugins(worker::Worker);

    app.run();
}

// didnt work
// store the agen positions (agent_id, grid_position)
// block the grid_positions where the agent is

// pending
// if more than 1 agent is selected 
// search another gridcell to assign
// assign the closer not assigned gridcell

// when assigning the new agentpos 
// order by closest worker to the new position
// by order assign the new position then the closest grid_cells

// workers cna operate machines to generate resources
// you only have one but can get more 
// automate things + rts mechanics