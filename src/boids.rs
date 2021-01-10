use bevy::{
    prelude::*,
    input::{
        keyboard::KeyCode,
        Input,
    },
};
use crate::bvh::{BVHNode, AABB};
use crate::utils::random_vec3;

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_boids.system())
            .add_system(update_boids.system());
    }
}

struct Boid;

fn update_boids(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Boid>>,
) {
    let vision_radius = 2.0_f32;
    // initialize bvh
    let bbox = AABB::new(Vec3::splat(-0.125), Vec3::splat(0.125));
    let mut data_and_boxes: Vec<(Transform, AABB)> = Vec::new();
    //let mut entities: Vec<Entity> = Vec::new();
    for transform in query.iter_mut() {
        data_and_boxes.push((transform.clone(), bbox.translated(&transform.translation)));
    //    entities.push(e);
    }
    let root = BVHNode::create(data_and_boxes.clone()).unwrap();

    // iterate over mutable query and update transforms
    for mut transform in query.iter_mut() {
        if let Some(neighbors) = root.get_in_radius(&transform.translation, vision_radius) {
            // rotate the boid somehow
            for neighbor in neighbors.iter() {

            }

            let cohesion = cohesion(&transform, &neighbors);
            transform.rotate(cohesion);

            let forward = transform.forward();
            transform.translation += forward * time.delta_seconds();
        }
    }
}

// Steer away from the closest neighbor
fn separation(boid_t: &Transform, neighbors: &Vec<Transform>) -> Quat {
    Quat::default()
}
// Look in the same direction as neighbors, Average of neighbors rotations
fn alignment(boid_t: &Transform, neighbors: &Vec<Transform>) -> Quat {
    Quat::default()
}
// Steer towards the geometric middle of the neighbors
fn cohesion(boid_t: &Transform, neighbors: &Vec<Transform>) -> Quat {
    // Geometric center of neighbors
    let mut position_sum = Vec3::zero();
    for neighbor in neighbors.iter() {
        position_sum += neighbor.translation;
    }
    let position_avg = position_sum / neighbors.len() as f32;

    // Look toward position_avg
    let heading = boid_t.forward();
    let position = boid_t.translation;
    let rotation = heading.cross(position_avg - position);

    Quat::from_axis_angle(rotation, rotation.length() * 0.1)
}

fn spawn_boids(
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        _asset_server: ResMut<AssetServer>,
) {
    let num_boids = 80;

    let mesh_handle_box = meshes.add(
        Mesh::from(
            shape::Box::new(1.0, 1.0, 1.0) 
        )
    );
    let material_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
 

    // Spawn random boids
    for _ in 0..num_boids {
        let mut transform = Transform::from_translation(random_vec3() * 8.0);
        transform.apply_non_uniform_scale(Vec3::splat(0.25));
        commands.spawn(PbrBundle {
                mesh: mesh_handle_box.clone(),
                material: material_handle.clone(),
                transform: transform,
                ..Default::default()
            })
            .with(Boid);
    }
}