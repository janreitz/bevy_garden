use crate::dynamics::RigidBody;
use bevy::{
    prelude::*,
    input::{
        keyboard::KeyCode,
        Input,
    },
};
use std::f32::consts::PI;

pub struct ThrusterPlugin;
impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_thruster_3d)
            .add_system(thruster_control)
            .add_system(thruster_3d_control);
    }
}

#[derive(Component)]
struct Thruster {
    // Direction is interpreted relative to Transform Component
    force: Vec3,
}

fn thruster_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Thruster, &mut RigidBody, &Transform)>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (thruster, mut rb, transform) in query.iter_mut() {
            // adjust force direction to RigidBody rotation 
            let resulting_force = transform.rotation.mul_vec3(thruster.force);
            rb.apply_force(resulting_force);
        }
    }
}

fn _spawn_test_thruster(
    mut commands:  Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let monkey_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/monkey.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    let mut transform = Transform::from_translation(Vec3::splat(4.0));
    transform.rotate(Quat::from_axis_angle(Vec3::unit_x(), PI / 8.0));
    commands
    .spawn(PbrBundle {
        mesh: monkey_handle,
        material: green_material,
        transform: transform,
        ..Default::default()
    })
    .with(RigidBody::default())
    .with(Thruster {
        force: Vec3::unit_y() * 20.0,
    })
    ;
}

#[derive(Component)]
struct Thruster3d{
    force_up: Vec3,
    force_left: Vec3,
    force_right: Vec3,
    force_forward: Vec3,
    force_back: Vec3,
    torque_clockwise: Vec3,
    torque_counter_clockwise: Vec3,
}

fn thruster_3d_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Thruster3d, &mut RigidBody, &Transform)>
) {
    for (thruster, mut rb, transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            let resulting_force = transform.rotation.mul_vec3(thruster.force_up);
            rb.apply_force(resulting_force);
        }
        if keyboard_input.pressed(KeyCode::W) {
            let resulting_force = transform.rotation.mul_vec3(thruster.force_forward);
            rb.apply_force(resulting_force);
        }
        if keyboard_input.pressed(KeyCode::S) {
            let resulting_force = transform.rotation.mul_vec3(thruster.force_back);
            rb.apply_force(resulting_force);
        }
        if keyboard_input.pressed(KeyCode::A) {

            let resulting_force = transform.rotation.mul_vec3(thruster.force_left);
            rb.apply_force(resulting_force);
        }
        if keyboard_input.pressed(KeyCode::D) {

            let resulting_force = transform.rotation.mul_vec3(thruster.force_right);
            rb.apply_force(resulting_force);
        }
        if keyboard_input.pressed(KeyCode::Q) {

            let resulting_torque = transform.rotation.mul_vec3(thruster.torque_counter_clockwise);
            rb.apply_torque(resulting_torque);
        }
        if keyboard_input.pressed(KeyCode::E) {

            let resulting_torque = transform.rotation.mul_vec3(thruster.torque_clockwise);
            rb.apply_torque(resulting_torque);
        }
    }
}

fn spawn_test_thruster_3d(
    mut commands:  Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let monkey_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/monkey.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    let transform = Transform::from_translation(Vec3::splat(4.0));
    commands
    .spawn(PbrBundle {
        mesh: monkey_handle,
        material: green_material,
        transform: transform,
        ..Default::default()
    })
    .with(RigidBody::new(
         1.0,
        Mat3::from_cols(
            Vec3::new(1.0,5.0, 0.0),
            Vec3::new(5.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
        Vec3::zero(),
        Vec3::zero(),
        Vec3::zero(),
    ))
    .with(Thruster3d {
        force_up: Vec3::unit_y() * 20.0,
        force_left: Vec3::unit_x() * 5.0,
        force_right: Vec3::unit_x() * -5.0,
        force_forward: Vec3::unit_z() * 5.0,
        force_back: Vec3::unit_z() * -5.0,
        torque_clockwise: Vec3::unit_y() * -1.0,
        torque_counter_clockwise: Vec3::unit_y() * 1.0,
    })
    ;
}