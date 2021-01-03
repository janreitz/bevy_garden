use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {size: 8.0})),
            material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            transform: Transform::from_translation(Vec3::new(4.0,0.0,4.0)),
            ..Default::default()
        })
        // Camera
        .spawn((PanOrbitCamera::default(),))
        .with_bundle(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_xyzw(-0.3, -0.5,-0.3,0.5).normalize(), Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0,4.0)),
            ..Default::default()
        });
}

/// Tags an entity as capable of panning and orbiting.
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
#[derive(Default)]
struct InputState {
    pub reader_motion: EventReader<MouseMotion>,
    pub reader_scroll: EventReader<MouseWheel>,
}

/// Pan the camera with LHold or scrollwheel, orbit with rclick.
fn pan_orbit_camera(
    time: Res<Time>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    ev_motion: Res<Events<MouseMotion>>,
    mousebtn: Res<Input<MouseButton>>,
    ev_scroll: Res<Events<MouseWheel>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut translation = Vec2::zero();
    let mut rotation_move = Vec2::default();
    let mut scroll = 0.0;
    let dt = time.delta_seconds();

    if mousebtn.pressed(MouseButton::Right) {
        for ev in state.reader_motion.iter(&ev_motion) {
            rotation_move += ev.delta;
        }
    } else if mousebtn.pressed(MouseButton::Left) {
        // Pan only if we're not rotating at the moment
        for ev in state.reader_motion.iter(&ev_motion) {
            translation += ev.delta;
        }
    }

    for ev in state.reader_scroll.iter(&ev_scroll) {
        scroll += ev.y;
    }

    // Either pan+scroll or arcball. We don't do both at once.
    for (mut camera, mut trans) in query.iter_mut() {
        if rotation_move.length_squared() > 0.0 {
            let window = windows.get_primary().unwrap();
            let window_w = window.width() as f32;
            let window_h = window.height() as f32;

            // Link virtual sphere rotation relative to window to make it feel nicer
            let delta_x = rotation_move.x / window_w * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window_h * std::f32::consts::PI;

            let delta_yaw = Quat::from_rotation_y(delta_x);
            let delta_pitch = Quat::from_rotation_x(delta_y);

            trans.translation =
                delta_yaw * delta_pitch * (trans.translation - camera.focus) + camera.focus;

            let look = Mat4::face_toward(trans.translation, camera.focus, Vec3::new(0.0, 1.0, 0.0));
            trans.rotation = look.to_scale_rotation_translation().1;
        } else {
            // The plane is x/y while z is "up". Multiplying by dt allows for a constant pan rate
            let mut translation = Vec3::new(-translation.x * dt, translation.y * dt, 0.0);
            camera.focus += translation;
            translation.z -= scroll;
            trans.translation += translation;
        }
    }
}

// // Spawn a camera like this:

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn((PanOrbitCamera::default(),))
//         .with_bundle(Camera3dComponents {
//             ..Default::default()
//         });
// }

fn main() {
    App::build()
        .init_resource::<InputState>()
        .add_resource(Msaa { samples: 4})
        .add_resource(WindowDescriptor {
            title: String::from("Garden"),
            width: 1600.0,
            height: 1600.0,
            // This is struct update syntax, filling out the remainder 
            // of the struct with default values as provided by this struct
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Startup systems are called only one, at startup
        .add_startup_system(
            // calling `system()` on a function turns it into a system
            setup.system()
        )
        .add_system(pan_orbit_camera.system())
        .run();
}