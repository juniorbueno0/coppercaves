use bevy::prelude::*;

struct CameraSettings {
    mov_speed: f32,
    zoom_speed: f32
}

const CAMERA: CameraSettings = CameraSettings {
    mov_speed: 26.,
    zoom_speed: 0.33
};

#[derive(Debug, Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct MainCameraActualPosition(pub Vec2);

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainCameraActualPosition(vec2(0.0,0.0)));
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (move_camera, zoom_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d { ..Default::default() }, 
        MainCamera
    ));
}

fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera_position: ResMut<MainCameraActualPosition>,
    camera_query: Single<&mut Transform, With<MainCamera>>
) {
    let mut transform = camera_query.into_inner();

    for key in input.get_pressed() {
        match key {
            KeyCode::KeyW => { transform.translation.y += CAMERA.mov_speed * time.delta_secs(); },
            KeyCode::KeyS => { transform.translation.y -= CAMERA.mov_speed * time.delta_secs(); },
            KeyCode::KeyD => { transform.translation.x += CAMERA.mov_speed * time.delta_secs(); },
            KeyCode::KeyA => { transform.translation.x -= CAMERA.mov_speed * time.delta_secs(); },
            _ => {}
        }
        camera_position.0 = Vec2::new(transform.translation.x, transform.translation.y);
    }
}

fn zoom_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    camera_query: Single<&mut Projection, With<MainCamera>>
) {
    let mut orthp = camera_query.into_inner();

    for key in input.get_pressed() {
        match key {
            KeyCode::KeyQ => {
                match *orthp {
                    Projection::Orthographic(ref mut orthographic) => {
                        orthographic.scale -= CAMERA.zoom_speed * time.delta_secs();
                    },
                    _ => {}
                }
            },
            KeyCode::KeyE => {
                match *orthp {
                    Projection::Orthographic(ref mut orthographic) => {
                        orthographic.scale += CAMERA.zoom_speed * time.delta_secs();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}