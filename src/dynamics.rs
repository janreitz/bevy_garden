use bevy::prelude::*;

pub struct DynamicsPlugin;
impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Gravity>()
            .add_startup_system(spawn_test_box.system())
            .add_system(gravity.system())
            .add_system(dynamic_simulation.system());
    }
}

pub struct RigidBody {
    mass: f32,
    inertia: Vec3,
    velocity: Vec3,
    angular_velocity: Vec3,
    force: Vec3,
    torque: Vec3,
}

impl Default for RigidBody {
    fn default() -> RigidBody {
        RigidBody {
            mass: 1.0,
            inertia: Vec3::one(),
            velocity: Vec3::new(0.0,0.0,0.0),
            angular_velocity: Vec3::new(0.0,0.0,0.0),
            force: Vec3::new(0.0,0.0,0.0),
            torque: Vec3::new(0.0,0.0,0.0),
        }
    }
}

impl RigidBody {
    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }
}

fn dynamic_simulation(
    time: Res<Time>,
    mut query: Query<(&mut RigidBody, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    // Update velocities
    for (mut rigid_body, _transform) in query.iter_mut() {
        let acc = rigid_body.force / rigid_body.mass * dt;
        rigid_body.velocity += acc;
        // reset force
        rigid_body.force = Vec3::zero();

    } 
    // Update positions
    for (rigid_body, mut transform) in query.iter_mut() {
        transform.translation += rigid_body.velocity * dt;
    }
}

pub struct Gravity{
    acceleration: Vec3,
}

impl Default for Gravity {
    fn default() -> Gravity {
        Gravity {
            acceleration: Vec3::unit_y() * -9.81,
        }
    }
}

fn gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<&mut RigidBody>,
) {
    let dt = time.delta_seconds();
    // Update velocities
    for mut rigid_body in query.iter_mut() {
        rigid_body.velocity += gravity.acceleration * dt;
    }
} 

fn spawn_test_box(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
    let material_handle = materials.add(Color::BLUE.into());
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
        ..Default::default()
    })
    .with(RigidBody::default());
}