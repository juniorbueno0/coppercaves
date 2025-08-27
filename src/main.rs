use bevy::{image::ImageSamplerDescriptor, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::WindowResolution};
use bevy_northstar::prelude::*;

mod grid;
mod mouse;
mod world;
mod player;
mod camera;
mod worker;
mod ui_game;

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