use bevy::{image::ImageSamplerDescriptor, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::WindowResolution};
use bevy_northstar::prelude::*;

mod grid;
mod mouse;
mod world;
mod camera;
mod worker;

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

    app.add_plugins(world::MyWorldPlugin);

    // app.add_plugins(grid::MyGridPlugin);
    // app.add_plugins(worker::Worker);

    app.add_systems(Update, (draw_player, move_player, input));

    app.run();
}

// limit the world to a specific maz size;
// the grid has to be generated at the start;
// later use the same rules to generate the other grid (the rendered one) hoping this helps with performance!

fn draw_player(query: Query<&Transform, With<AgentPos>>, mut gizmos: Gizmos) {
    for transform in &query {
        // Draw a simple circle at the agent's position.
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            4.0,                             // Radius of the circle.
            Color::srgba_u8(0, 255, 0, 255), // Color of the circle.
        );
    }
}

fn move_player(
    mut query: Query<(Entity, &mut AgentPos, &NextPos, &mut Transform)>,
    mut commands: Commands,
) {
    let offset = Vec3::new(-384.0, -288.0, 0.0); // Offset to center on the world.

    for (entity, mut agent_pos, next_pos, mut transform) in &mut query {
        // Set the transform position to the agent's position in the grid.
        transform.translation = Vec3::new(
            next_pos.0.x as f32 * 12.0 + offset.x, // Align with the grid cell size.
            next_pos.0.y as f32 * 12.0 + offset.y,
            0.0,
        );

        // Update the agent's position to the next position.
        agent_pos.0 = next_pos.0;

        // Now we remove the NextPos component from the player to consume it.
        // This is important to get the next updated position in the path.
        commands.entity(entity).remove::<NextPos>();
    }
}

fn input(
    input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera>>,
    player: Single<Entity, With<AgentPos>>,
    grid: Single<&mut CardinalGrid>,
    mut commands: Commands,
) {
    let window = window.into_inner();
    let (camera, camera_transform, _) = camera.into_inner();
    let player = player.into_inner();

    let clicked_tile = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        .map(|cursor_position| {
            let offset = Vec2::new(-384.0, -288.0);
            let cursor_position = cursor_position - offset;
            UVec3::new(
                (cursor_position.x / 12.0).round() as u32,
                (cursor_position.y / 12.0).round() as u32,
                0,
            )
        });
    // Most of this isn't important for using the crate and is standard Bevy usage.
    // We just want to demonstrate how to use the pathfinding system with a mouse click.
    if input.just_pressed(MouseButton::Left) {
        if let Some(goal) = clicked_tile {
            // This is the important bit here.
            // We insert a Pathfind component with the goal position.
            // The pathfinding system will insert a NextPos component
            // on the next frame.
            commands.entity(player).insert(Pathfind::new(goal));
        }
    }

    // Right click to toggle the navigation state of the clicked tile.
    // This demonstrates how to dynamically change the grid's navigation data.
    if input.just_pressed(MouseButton::Right) {
        if let Some(position) = clicked_tile {
            let mut grid = grid.into_inner();

            if let Some(nav) = grid.nav(position) {
                if !matches!(nav, Nav::Impassable) {
                    // If the cell is passable, we set it to impassable.
                    grid.set_nav(position, Nav::Impassable);
                } else {
                    // If the cell is impassable, we set it to passable with a cost of 1.
                    grid.set_nav(position, Nav::Passable(1));
                }
            } else {
                return;
            }
            // You must call `build` after modifying the grid to update the internal state.
            grid.build();
        }
    }
}