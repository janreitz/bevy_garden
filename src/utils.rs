use bevy::prelude::*;
use rand::random;

fn random_vec3() -> Vec3 {
    Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
}