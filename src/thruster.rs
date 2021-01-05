use crate::dynamics::RigidBody;
use bevy::{
    prelude::*,
    input::{
        keyboard::KeyCode,
        Input,
    },
};

pub struct ThrusterPlugin;
impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_test_thruster.system())
            .add_system(thruster_control.system());
    }
}

struct Thruster {
    // Direction is interpreted relative to Transform Component
    force: Vec3,
}

fn thruster_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Thruster, &mut RigidBody)>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (thruster, mut rb) in query.iter_mut() {
            rb.apply_force(thruster.force);
        }
    }
}

fn spawn_test_thruster(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let monkey_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/monkey.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    commands
    .spawn(PbrBundle {
        mesh: monkey_handle,
        material: green_material,
        transform: Transform::from_translation(Vec3::splat(4.0)),
        ..Default::default()
    })
    .with(RigidBody::default())
    .with(Thruster {
        force: Vec3::unit_y(),
    })
    ;
}