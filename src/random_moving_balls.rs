use bevy::prelude::*;
use rand::random;
use crate::bvh::{BVHNode, AABB};

pub struct RandomMovingBallsPlugin;
impl Plugin for RandomMovingBallsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_balls.system())
            .add_system(move_balls.system())
            .add_system(test_color_balls_bvs.system());
    }
}

struct RandomMovingBall;
struct FocusBall;

// struct BallBoundingBox{
//     min: Vec3,
//     max: Vec3,
// }
// impl Default for BallBoundingBox {
//     fn default() -> BallBoundingBox {
//         BallBoundingBox {
//             min: Vec3::splat(0.0),
//             max: Vec3::splat(8.0),
//         }
//     }
// }

fn spawn_balls(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    let num_balls = 40;

    let mesh_handle = meshes.add(Mesh::from(shape::Icosphere{radius: 1.0, subdivisions: 10}));
    let material_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());

    for _ in 0..num_balls {
        let position = random_vec3() * 8.0;
        spawn_ball(commands, mesh_handle.clone(), material_handle.clone(), position, 0.25);
    }
    // The last spawned ball get this label attached
    commands.with(FocusBall);
}

fn move_balls(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RandomMovingBall>>
) {
    let dt = time.delta_seconds();
    let offset = Vec3::splat(-0.5);
    for mut transform in query.iter_mut() {
        transform.translation += (random_vec3() + offset)  * dt * 5.0;
    }
}

fn test_color_balls_bvs(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_set: QuerySet<(
        Query<(Entity, &Transform, &mut Handle<StandardMaterial>), (With<RandomMovingBall>, Without<FocusBall>)>,
        Query<(&Transform, &mut Handle<StandardMaterial>), With<FocusBall>>,
    )>

) {
    let green_handle = materials.add(Color::GREEN.into());
    let red_handle = materials.add(Color::RED.into());
    let neutral_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());

    // initialize bvh
    let bbox = AABB::new(Vec3::splat(-2.5), Vec3::splat(2.5));
    let mut data_and_boxes: Vec<(Entity, AABB)> = Vec::new();
    for (e, transform, mut material) in query_set.q0_mut().iter_mut() {
        data_and_boxes.push((e, bbox.translated(&transform.translation)));
        *material = neutral_handle.clone();
    }
    let root = BVHNode::create(data_and_boxes).unwrap();

    let mut focus_ball_position = Vec3::zero();
    for (transform, mut material) in query_set.q1_mut().iter_mut() {
        focus_ball_position = transform.translation.clone();
        *material = green_handle.clone();
    }
    
    if let Some(closest_entity) = root.get_closest(&focus_ball_position){
        let mut material = query_set.q0_mut().get_component_mut::<Handle<StandardMaterial>>(closest_entity.0).unwrap();    
        *material = red_handle.clone();
    } else {
        println!("No closest entity found");
    }
    

}

fn random_vec3() -> Vec3 {
    Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
}

fn spawn_ball(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
    radius: f32,
) {
    let mut transform = Transform::from_translation(position);
    transform.apply_non_uniform_scale(Vec3::new(radius, radius, radius));
    commands.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: transform,
            ..Default::default()
        })
        .with(RandomMovingBall);
}