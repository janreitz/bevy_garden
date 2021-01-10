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
    for (mut transform, _) in data_and_boxes.iter_mut() {
        if let Some(neighbors) = root.get_in_radius(&transform.translation, vision_radius) {
            for neighbor in neighbors.iter() {

            }
        }
    }
}

// Look away from neighbors
fn separation(boid_t: &Transform, neighbors: &Vec<Transform>) -> Transform {
    Transform::default()
}
// Look in the same direction as neighbors
fn alignment(boid_t: &Transform, neighbors: &Vec<Transform>) -> Transform {
    Transform::default()
}
// Look towards neighbors
fn cohesion(boid_t: &Transform, neighbors: &Vec<Transform>) -> Transform {
    Transform::default()
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