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
    weather: Res<Weather>,
    time: Res<Time>,
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Transform), With<SnowFlake>>
) {
    if weather.weather_type != WeatherType::Snow {
        return;
    }

    // Spawn new snowflakes and move existing ones downwards
    // Randomize x,y position and size
    let position = Vec3::new(random::<f32>() * 8.0, 4.0, random::<f32>() * 8.0);
    let mut rng = thread_rng();
    let normal = Normal::new(0.001, 0.001).unwrap();
    let volume: f32 = normal.sample(&mut rng);
    let radius = volume.powf(0.33);
    spawn_snowflake(commands, meshes, materials, position, radius);

    let dt = time.delta_seconds();
    for (entity, mut transform) in query.iter_mut() {
        // Remove Snowflakes once they are below the ground
        if transform.translation.y < 0.0 {
            commands.despawn(entity);
        }
        transform.translation += Vec3::unit_y() * -dt;
    }
}

fn spawn_snowflake(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    radius: f32,
) {
    commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere{radius: radius, subdivisions: 10})),
            material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(SnowFlake);
}