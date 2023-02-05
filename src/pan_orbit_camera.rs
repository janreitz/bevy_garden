use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

pub struct PanOrbitCameraPlugin;
impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        // Resources need to be initialized before use
        app.init_resource::<InputState>()
            .add_system(pan_orbit_camera)
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands:  Commands,
) {
    commands
    // Camera
    .spawn((PanOrbitCamera::default(),))
    .with_bundle(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_xyzw(-0.3, -0.5,-0.3,0.5).normalize(), Vec3::new(-7.0, 20.0, 4.0),
    )),
    ..Default::default()
    });
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::zero(),
        }
    }
}

/// Hold readers for events
#[derive(Resource)]
struct InputState<'a> {
    pub reader_motion:  EventReader<'a, 'a, MouseMotion>,
    pub reader_scroll: EventReader<'a, 'a, MouseWheel>,
}

/// Pan the camera with LHold or scrollwheel, orbit with rclick.
fn  pan_orbit_camera<'a>(
    time: Res<Time>,
    windows: Res<Windows>,
    mut state: ResMut<'a, InputState>,
    mousebtn: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut translation_mouse_delta = Vec2::zero();
    let mut rotation_mouse_delta = Vec2::default();
    let mut scroll = 0.0;
    let dt = time.delta_seconds();

    if mousebtn.pressed(MouseButton::Right) {
        for ev in state.reader_motion.iter() {
            rotation_mouse_delta += ev.delta;
        }
    } else if mousebtn.pressed(MouseButton::Left) {
        // Pan only if we're not rotating at the moment
        for ev in state.reader_motion.iter() {
            translation_mouse_delta += ev.delta;
        }
    }

    for ev in state.reader_scroll.iter() {
        scroll += ev.y;
    }

    // Either pan+scroll or arcball. We don't do both at once.
    for (mut camera, mut cam_transform) in query.iter_mut() {
        if rotation_mouse_delta.length_squared() > 0.0 {
            let window = windows.get_primary().unwrap();
            let window_w = window.width() as f32;
            let window_h = window.height() as f32;

            // Link virtual sphere rotation relative to window to make it feel nicer
            let delta_x = rotation_mouse_delta.x / window_w * std::f32::consts::PI * 2.0;
            let delta_y = rotation_mouse_delta.y / window_h * std::f32::consts::PI;

            let delta_yaw = Quat::from_rotation_y(delta_x);
            let delta_pitch = Quat::from_rotation_x(delta_y);

            cam_transform.translation =
                delta_yaw * delta_pitch * (cam_transform.translation - camera.focus) + camera.focus;

            let look = Mat4::face_toward(cam_transform.translation, camera.focus, Vec3::new(0.0, 1.0, 0.0));
            cam_transform.rotation = look.to_scale_rotation_translation().1;
        } else {
            let forward_direction = cam_transform.forward().normalize();
            // The plane is x/y while z is "up". Multiplying by dt allows for a constant pan rate
            let pan_movement = Vec3::new(translation_mouse_delta.x * dt, translation_mouse_delta.y * dt, 0.0);
            camera.focus += pan_movement;
            cam_transform.translation += pan_movement;
            // Move in the direction the camera is facing
            let scroll_movement = forward_direction * -scroll;
            cam_transform.translation += scroll_movement;
        }
    }
}