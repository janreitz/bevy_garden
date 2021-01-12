use bevy::{prelude::*, ui::FlexSurface};
use crate::bvh::{BVHNode, AABB};
use crate::utils::random_vec3;

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(setup_ui.system())
            .add_startup_system(spawn_boids.system())
            .add_system(update_boids.system())
            .add_system(button_system.system());
    }
}

struct Boid;

fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        // root node
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            material: materials.add(Color::PINK.into()),
            ..Default::default()
        })
        .with_children(|parent|{ 
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        value: "Button".to_string(),
                        font: asset_server.load("fonts/Inconsolata.ttf"),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
        })
        .with_children(|parent|{ 
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        value: "Button".to_string(),
                        font: asset_server.load("fonts/Inconsolata.ttf"),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
        })
        .with_children(|parent|{ 
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        value: "Button".to_string(),
                        font: asset_server.load("fonts/Inconsolata.ttf"),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
        });
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn update_boids(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Boid>>,
) {
    let vision_radius = 4.0_f32;
    let dt = time.delta_seconds();
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
    for mut boid_transform in query.iter_mut() {
        if let Some(neighbor_transforms) = root.get_in_radius(&boid_transform.translation, vision_radius) {
            if neighbor_transforms.len() < 2 {
                // No effective neighbors, only comparing with self
            } else {
                let mut sum_heading = Vec3::zero();
                let mut sum_position = Vec3::zero();
                let mut min_distance = Vec3::splat(f32::MAX) ;
                for neighbor_transform in neighbor_transforms.iter() {
                    if *neighbor_transform == *boid_transform {
                        // Do not compare with self
                        continue;
                    }
                    sum_heading += neighbor_transform.forward();
                    sum_position += neighbor_transform.translation;
                    let distance = neighbor_transform.translation - boid_transform.translation;
                    if distance.length() < min_distance.length() 
                        // Avoid comparing with self
                        && 0.000001 < distance.length()  {
                        min_distance = distance;
                    }
                }
                let avg_position = sum_position / neighbor_transforms.len() as f32;
                let avg_heading = sum_heading / neighbor_transforms.len() as f32;

                // Separation
                // Steer away from the closest neighbor
                assert_ne!(min_distance, Vec3::splat(f32::MAX));
                // println!("boids::update_boids -> min_distance = {}", min_distance);
                // let rotation = boid_transform.forward().cross(min_distance);
                
                let heading = boid_transform.forward();
                let w = heading.cross(min_distance) * 1.0 * dt * 0.5 * 0.1;
                let w = Quat::from_xyzw(w.x, w.y, w.z, 0.0); 

                boid_transform.rotation = boid_transform.rotation + w.mul_quat(boid_transform.rotation);
                boid_transform.rotation.normalize();
                // assert!(rotation.is_finite());
                // assert_ne!(rotation, Vec3::zero());
                //let separation = Quat::from_axis_angle(rotation, 10.0 / rotation.length());
                //assert!(separation.is_finite());
                
                // Alignment
                // Look towards the same direction as neighbors
                let heading = boid_transform.forward();
                let w = heading.cross(avg_heading) * 1.0 * dt * 0.5 * 0.1;
                let w = Quat::from_xyzw(w.x, w.y, w.z, 0.0); 

                boid_transform.rotation = boid_transform.rotation + w.mul_quat(boid_transform.rotation);
                boid_transform.rotation.normalize();

                // Cohesion
                // Steer toward position_avg
                let heading = boid_transform.forward();
                let w = heading.cross(avg_position - boid_transform.translation) * 1.0 * dt * 0.5 * 0.1;
                let w = Quat::from_xyzw(w.x, w.y, w.z, 0.0); 

                boid_transform.rotation = boid_transform.rotation + w.mul_quat(boid_transform.rotation);
                boid_transform.rotation.normalize();
            }
            
            let forward = boid_transform.forward();
            assert!(forward.is_finite());
            boid_transform.translation += forward * dt;

            assert!(boid_transform.translation.is_finite());
            assert!(boid_transform.rotation.is_finite());
        }
    }
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

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}