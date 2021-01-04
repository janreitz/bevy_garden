use bevy::prelude::*;
use bevy::core::FixedTimestep;
use std::f32::consts::PI;
// use bevy::log::info;

pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_trees.system())
        //.add_system(tree_growth.system())
        .add_stage_after(stage::UPDATE, "fixed_update", SystemStage::parallel()
            .with_run_criteria(FixedTimestep::step(0.5))
            .with_system(tree_growth.system())
        );
    }
}

fn tree_growth(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_set: QuerySet<(
        // leaf_segments: 
        Query<(Entity, &mut TreeSegment, & Transform), With<Leaf>>,
        // all_segments: 
        Query<(& TreeSegment, &mut Transform)>,
    )>
) {
    let segment_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/cylinder.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    // Leaves always grow
    for (entity, mut segment, transform) in query_set.q0_mut().iter_mut() {
        // New Transform
        let mut new_transform = transform.clone();
        //new_transform.rotate(Quat::from_axis_angle(transform.forward(), PI/8.0));
        new_transform.translation += Vec3::unit_y() * 1.0;
        // Create new tree segment, which is a Leaf
        spawn_tree_segment(
            commands, 
            segment_handle.clone(), 
            green_material.clone(), 
            new_transform,
        );
        commands.remove_one::<Leaf>(entity);
        segment.children.push(commands.current_entity().unwrap());
    }
    
    for (mut _segment, _transform) in query_set.q1_mut().iter_mut() {
        // Update Thickness
        //transform.apply_non_uniform_scale(Vec3::new(1.01, 1.0, 1.01));
    }
}

fn create_trees(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let segment_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/cylinder.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    let mut transform = Transform::from_translation(Vec3::new(4.0, 1.0, 4.0 ));
    transform.apply_non_uniform_scale(Vec3::new(0.15,0.5,0.15));

    spawn_tree_segment(
        commands, 
        segment_handle, 
        green_material,  
        transform
    );
}

fn spawn_tree_segment(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        })
        .with(TreeSegment {_thickness: 1.0, children: Vec::new()})
        // New Segments are always leaves
        .with(Leaf{});
    
    commands.current_entity().unwrap()
}

struct Leaf;
struct _Pose {
  //  
}

struct TreeSegment {
    _thickness: f32,
    children: Vec<Entity>,
}

impl TreeSegment {
    //     // for child in self.children.iter_mut() {
    //     //     child.grow(commands);
    //     // }
    //     // // Leaves always grow
    //     // if self.is_leaf() {
    //     //     self.branch();
    //     // }
    //     // // Random branching, dependent on thickness
    //     // else if random_range::<f32>(0.0, 50.0) < 1.0/self.thickness.powi(2) {
    //     //     self.branch();
    //     // }
    
    //     // // Grow leaves
    //     // let mut rng = rand::thread_rng();
    //     // // The thinner the branch the more leaves it has
    //     // let num_leaves = (rng.gen::<f32>() * 15.0 / self.thickness) as i32;
    //     // let leave_diff = num_leaves - self.leaves.len() as i32; 
    
    //     // if leave_diff < 0 {
    //     //     // remove leaves
    //     //     for _ in 0..-leave_diff {
    //     //         self.leaves.pop();
    //     //     }
    //     // }
    //     // else if leave_diff > 0 {
    //     //     // add leaves
    //     //     let col1 = hsv(0.0,0.0,1.0);
    //     //     let col2 = hsv(1.0,1.0,1.0);
    
    //     //     for _ in 0..leave_diff {
    //     //         self.leaves.push(Leaf{
    //     //             orientation: rng.gen::<f32>() * 2.0 * PI,
    //     //             position: rng.gen::<f32>(),
    //     //             offset: rng.gen::<f32>() * 50.0,
    //     //             size: rng.gen::<f32>() * 15.0,
    //     //             color: col1.mix(&col2, rng.gen::<f32>()), 
    //     //         })
    //     //     }
    //     // }
    //     // self.update_thickness();
    // }
    

    // fn update_thickness(& mut self) {
    //     let mut sum_squared_thicknesses = 1.0;
    //     for child in self.children.iter() {
    //         sum_squared_thicknesses += child.thickness.powi(2);
    //     }
    //     self.thickness = sum_squared_thicknesses.sqrt();
    //     self.transform.apply_non_uniform_scale(Vec3::new(self.thickness, 0.0, 0.0))
    // }

    // fn branch(&mut self) {
    //     // Spawn a child
    //     // let my_shape = self.b - self.a;
    //     // let mut current_angle = 0.0;
    //     // if my_shape.x == 0.0 {
    //     //     current_angle = (my_shape.x/my_shape.y).tan() 
    //     // }
    //     // let mean_regression = 0.02;
    //     // let angle =  mean_regression * current_angle + (1.0 - mean_regression) * deg_to_rad(random_range::<f32>(-10.0, 10.0)); 
    //     // let new_shape = my_shape.rotate(angle); 
    //     // let branch = Node::new(self.b, self.b + new_shape);
    //     // self.children.push(branch)
    // }
}

