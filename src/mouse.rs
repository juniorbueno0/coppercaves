use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::MainCamera, world::TILESIZE};

#[derive(Resource, Debug)]
pub struct MyWorldCoords(pub Vec2);

#[derive(Resource, Debug)]
pub struct GridClicked{ position: UVec3 }

#[derive(Component)]
#[require(Sprite, Transform)]
struct MousePixelPosition;

pub struct MyMousePlugin;

impl Plugin for MyMousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyWorldCoords(Vec2 { x:0., y:0. }));
        app.insert_resource(GridClicked{ position: UVec3::new(0, 0, 0) });

        app.add_systems(Startup, setup);
        app.add_systems(Update,(cursor_to_world_position, mouse_pixel_position, grid_click_coords));
    }
}

fn setup(mut commands:Commands) {
    // pixel where the player is selecting
    commands.spawn((
        MousePixelPosition,
        Transform {
            translation: Vec3 { x: 0., y: 0., z: 0. },
            rotation: Quat::from_xyzw(0., -0., -0., 0.),
            scale: Vec3 { x: 1., y: 1., z: 1. }
        },
        Sprite { 
            color: Color::srgba(0.8, 0.8,0.8, 0.3),
            custom_size: Some(Vec2::new(1.,1.)), 
            ..default() 
        }
    ));
}

fn cursor_to_world_position(
    mut mycoords: ResMut<MyWorldCoords>,
    mut cursor_events: EventReader<CursorMoved>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(_) = cursor_events.read().last() else { return };

    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        let x_value = world_position.x.round() as i32;
        let y_value = world_position.y.round() as i32;
        
        mycoords.0 = Vec2::new(x_value as f32, y_value as f32);
    }
}

fn mouse_pixel_position(
    pixel: Res<MyWorldCoords>,
    mut pixel_transform: Query<&mut Transform, With<MousePixelPosition>>
) {
    if let Ok(mut tf) = pixel_transform.single_mut() {
        tf.translation = Vec3::new(pixel.0.x,pixel.0.y, 4.);
    };
}

fn grid_click_coords(
    window: Single<&Window>,
    mut clik_position: ResMut<GridClicked>,
    camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera>>
) {
    let window = window.into_inner();
    let (camera, camera_transform, _) = camera.into_inner();

    let clicked_tile: Option<UVec3> = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        .map(|cursor_position| {
            let cursor_position = cursor_position;
            UVec3::new(
                (cursor_position.x / TILESIZE as f32).round() as u32,
                (cursor_position.y / TILESIZE as f32).round() as u32,
                0,
            )
        });

    match clicked_tile {
        Option::Some(position) => { clik_position.position = position; },
        Option::None => {}
    }
}