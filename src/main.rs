use bevy::prelude::*;

mod pan_orbit_camera;
use pan_orbit_camera::*;
mod tree;
use tree::*;
mod weather;
use weather::*;
mod dynamics;
use dynamics::*;
mod thruster;
use thruster::*;

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
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::rgb_u8(153,50,204).into()),
            transform: Transform::from_translation(Vec3::new(4.0,0.5,4.0)),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
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
        .add_plugin(TreePlugin)
        .add_plugin(WeatherPlugin)
        .add_plugin(DynamicsPlugin)
        .add_plugin(ThrusterPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        // Startup systems are called only one, at startup
        .add_startup_system(
            // calling `system()` on a function turns it into a system
            setup.system()
        )
        .run();
}