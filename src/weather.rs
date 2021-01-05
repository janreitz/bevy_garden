use bevy::prelude::*;
use rand::random;
use rand::thread_rng;
use rand_distr::{Distribution,Normal};
   

pub struct WeatherPlugin;
impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Weather>()
            .add_system(snow_simulation.system());
    }
}

#[derive(PartialEq)]
enum WeatherType {
    Snow,
    _Rain,
    _Sun,
}

impl Default for WeatherType{
    fn default() -> WeatherType { WeatherType::Snow }
}

#[derive(Default)]
struct Weather {
    weather_type: WeatherType,
}

struct SnowFlake;

fn snow_simulation(
    mut enough_snowflakes_spawned: Local<bool>,
    weather: Res<Weather>,
    time: Res<Time>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Transform, With<SnowFlake>>
) {
    if weather.weather_type != WeatherType::Snow {
        return;
    }

    let spawn_height = 2.0;
    let snowflakes_per_iteration = 1;

    // Move existing Snowflakes ones downwards
    let dt = time.delta_seconds();
    for mut transform in query.iter_mut() {
        // Reuse Snowflakes once they are below the ground
        if transform.translation.y < 0.0 {
            transform.translation += Vec3::new(0.0, spawn_height, 0.0);
            // Once the first snowflake hits the ground, enough have spawned
            *enough_snowflakes_spawned = true;
        }
        transform.translation += Vec3::unit_y() * -dt;
    }

    // Spawn new snowflakes
    if !*enough_snowflakes_spawned {
        let mesh_handle = meshes.add(Mesh::from(shape::Icosphere{radius: 1.0, subdivisions: 10}));
        let material_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());

        for _ in 0..snowflakes_per_iteration {
            // Randomize x,y position and Snowflake Volume
            // *total_snowflakes += 1;
            // println!("Spawning Snowflake, total snowflakes: {}", *total_snowflakes);
            let position = Vec3::new(random::<f32>() * 8.0, spawn_height, random::<f32>() * 8.0);
            let mut rng = thread_rng();
            let normal = Normal::new(0.001, 0.001).unwrap();
            let volume: f32 = normal.sample(&mut rng);
            let radius = volume.powf(0.33);
            spawn_snowflake(commands, mesh_handle.clone(), material_handle.clone(), position, radius);
        }
    }

}

fn spawn_snowflake(
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
        .with(SnowFlake);
}