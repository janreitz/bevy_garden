use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};
use rand::random;
use crate::bvh::{BVHNode, AABB};

pub struct RandomMovingBallsPlugin;
impl Plugin for RandomMovingBallsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<MyMaterial>()
            .add_startup_system(setup_transparent_material.system())
            .add_startup_system(spawn_balls.system())
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
    mut my_materials: ResMut<Assets<MyMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    asset_server: ResMut<AssetServer>,
) {
    let num_balls = 40;

    let mesh_handle_box = meshes.add(
        Mesh::from(
            shape::Box::new(1.0, 1.0, 1.0) 
        )
    );
    let mesh_handle_ball = meshes.add(
        Mesh::from(
            shape::Icosphere{
                radius: 1.0, 
                subdivisions: 10
            }
        )
    );
    let material_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());

    // Spawn random boxes
    for _ in 0..num_balls {
        let mut transform = Transform::from_translation(random_vec3() * 8.0);
        transform.apply_non_uniform_scale(Vec3::splat(0.25));
        commands.spawn(PbrBundle {
                mesh: mesh_handle_box.clone(),
                material: material_handle.clone(),
                transform: transform,
                ..Default::default()
            })
            .with(RandomMovingBall);
    }

    // Create a new shader pipeline with shaders loaded from the asset directory
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/hot.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/hot.frag")),
    }));

    let my_material = my_materials.add(MyMaterial {
        color: Color::rgb(0.0, 0.8, 0.0),
    });

    commands
        .with(FocusBall)
        .with_children( |parent| {
                parent.spawn(MeshBundle {
                    mesh: mesh_handle_ball,
                    render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                        pipeline_handle,
                    )]),
                    transform: Transform::from_scale(Vec3::splat(8.0)),
                    ..Default::default()
                })
                .with(my_material);
            }
    );
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
    let blue_handle = materials.add(Color::BLUE.into());
    let neutral_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());

    // initialize bvh
    let bbox = AABB::new(Vec3::splat(-0.125), Vec3::splat(0.125));
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
    
    if let Some(entities) = root.get_in_radius(&focus_ball_position, 2.0){
        for e in entities.iter() {
            let mut material = query_set.q0_mut().get_component_mut::<Handle<StandardMaterial>>(*e).unwrap();    
            *material = blue_handle.clone();
        }
    } else {
        println!("No entities in radius");
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

// Transparent Material
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
struct MyMaterial {
    pub color: Color,
}

fn setup_transparent_material(
    asset_server: ResMut<AssetServer>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to our shader
    render_graph.add_system_node(
        "my_material",
        AssetRenderResourcesNode::<MyMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("my_material", base::node::MAIN_PASS)
        .unwrap();
}