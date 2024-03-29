use bevy::prelude::*;

mod pan_orbit_camera;
use pan_orbit_camera::*;
mod tree;
use tree::*;
// mod weather;
// use weather::*;
mod dynamics;
use dynamics::*;
mod thruster;
use thruster::*;
mod fps_indicator;
use fps_indicator::*;
// mod collision_detection;
// use collision_detection::*;
mod random_moving_balls;
use random_moving_balls::*;
mod bvh;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Plane
        // .spawn(PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Plane {size: 8.0})),
        //     material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
        //     transform: Transform::from_translation(Vec3::new(4.0,0.0,4.0)),
        //     ..Default::default()
        // })
        // Light
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0,4.0)),
            ..Default::default()
        });
}


fn main() {
    App::build()
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
        .add_plugin(PanOrbitCameraPlugin)
        //.add_plugin(TreePlugin)
        //.add_plugin(WeatherPlugin)
        //.add_plugin(DynamicsPlugin)
        //.add_plugin(ThrusterPlugin)
        .add_plugin(RandomMovingBallsPlugin)
        .add_plugin(OnScreenFpsPlugin::new(OnScreenFpsConfig {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    min: Val::Px(1.0),
                    max: Val::Px(1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 22.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_system(bevy::window::close_on_esc)
        // Startup systems are called only one, at startup
        .add_startup_system(
            // calling `system()` on a function turns it into a system
            setup
        )
        .run();
}