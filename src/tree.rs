use bevy::prelude::*;

pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_trees.system())
            .add_system(tree_growth.system());
    }
}

fn create_trees(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tree_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/cylinder.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    commands
        .spawn(PbrBundle {
            mesh: tree_handle,
            material: green_material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(4.0, 1.0, 4.0 ));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
}


fn tree_growth(
    mut query: Query<&mut Tree>,
) {
    for mut tree in query.iter_mut() {
        tree.grow();
    }
}

struct Leaf {

}

#[derive(Bundle)]
struct TreeSegment {
    children: Vec<Node>,
    leaves: Vec<Leaf>,
    thickness: f32,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
}

impl TreeSegment {
    fn grow(&mut self) {

    }
}

struct Tree {
    root: TreeSegment,
}

impl Tree {
    fn grow(&mut self) {
        self.root.grow();
    }
}