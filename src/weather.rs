use bevy::prelude::*;

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
    Rain,
    Sun,
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
    mut query: Query<&mut Transform, With<SnowFlake>>
) {
    if weather.weather_type != WeatherType::Snow {
        return;
    }
    
    // Spawn new snowflakes and move existing ones downwards
    let position = Vec3::new(2.0, 2.0, 2.0);
    spawn_snowflake(commands, meshes, materials, position);

    let dt = time.delta_seconds();
    for mut transform in query.iter_mut() {
        transform.translation += Vec3::unit_y() * -dt;
    }
}

fn spawn_snowflake(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere{radius: 0.1, subdivisions: 10})),
            material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(SnowFlake);
}