use bevy::prelude::*;

mod pan_orbit_camera;
use pan_orbit_camera::*;
mod fps_indicator;
use fps_indicator::*;
mod boids;
use boids::*;
mod bvh;
mod utils;

fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0,4.0)),
            ..Default::default()
        });
}


fn main() {
    App::build()
        .add_resource(Msaa { samples: 4})
        .add_resource(WindowDescriptor {
            title: String::from("Boids!"),
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
        //.add_plugin(RandomMovingBallsPlugin)
        .add_plugin(BoidsPlugin)
        .add_plugin(OnScreenFpsPlugin::new(OnScreenFpsConfig {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(1.0),
                    left: Val::Px(1.0),
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
        .add_system(bevy::input::system::exit_on_esc_system.system())
        // Startup systems are called only one, at startup
        .add_startup_system(
            // calling `system()` on a function turns it into a system
            setup.system()
        )
        .run();
}