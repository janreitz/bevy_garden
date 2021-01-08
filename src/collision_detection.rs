use bevy::prelude::*;
use std::collections::HashMap;
use crate::bvh::BVH;

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // I keep the collision data in a spatial hash 
        // to reduce the number of comparisons
        app.init_resource::<SpatialHash>()
            .add_startup_system(test_spawn_colliding_bodies.system())
            // * Copy collidable data into the spatial hash
            .add_system(rebuild_spatial_hash.system())
            // * Do the comparisons for each cell 
            // * write back to the ECS
            .add_system(collision_detection.system())
            .add_system(test_color_according_to_collision.system());
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox{
    min: Vec3,
    max: Vec3,
    current_corner: u8,
}

impl Default for BoundingBox {
    fn default() -> BoundingBox {
        BoundingBox{
            min: Vec3::splat(-0.5),
            max: Vec3::splat(0.5),
            current_corner: 0,
        }
    }
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox {
            min,
            max,
            current_corner: 0
        }
    }

    pub fn transformed(&self, transform: &Transform) -> BoundingBox {
        self.translated(&transform.translation).rotated(&transform.rotation)
        // Should I also scale the bounding box?
    }
    // Are these new allocations expensive?
    pub fn translated(&self, translation: &Vec3) -> BoundingBox {
        BoundingBox::new(self.min + *translation, self.max + *translation)
    }

    pub fn rotated(&self, rotation: &Quat) -> BoundingBox {
        BoundingBox::new(rotation.mul_vec3(self.min), rotation.mul_vec3(self.max))
    }
}

impl Iterator for BoundingBox {
    type Item = Vec3;

    fn next(&mut self) -> Option<Vec3> {
        let corner: Option<Vec3>;
        match self.current_corner {
            0 => corner = Some(self.min),
            1 => corner = Some(Vec3::new(
                self.max.x,
                self.min.y,
                self.min.z
            )),
            2 => corner = Some(Vec3::new(
                self.max.x,
                self.max.y,
                self.min.z
            )),
            3 => corner = Some(Vec3::new(
                self.min.x,
                self.max.y,
                self.min.z
            )),
            4 => corner = Some(Vec3::new(
                self.max.x,
                self.min.y,
                self.max.z
            )),
            5 => corner = Some(Vec3::new(
                self.min.x,
                self.min.y,
                self.max.z
            )),
            6 => corner = Some(Vec3::new(
                self.min.x,
                self.max.y,
                self.max.z
            )),
            7 => corner = Some(self.max),
            _ => corner = None,
        }
        self.current_corner += 1;
        corner
    }
}

fn intersects(box_1: &BoundingBox, box_2: &BoundingBox) -> bool {
    if box_1.max.x < box_2.min.x { return false; }
    if box_1.min.x > box_2.max.x { return false; }
    if box_1.max.y < box_2.min.y { return false; }
    if box_1.min.y > box_2.max.y { return false; }
    if box_1.max.z < box_2.min.z { return false; }
    if box_1.min.z > box_2.max.z { return false; }
    return true;
}

struct SpatialHash {
    hash: HashMap<(u16, u16, u16), HashMap<Entity, (Collidable, Transform)>>,
    // Make sure the cell_length is larger than the biggest BoundingBox
    cell_length: f32,
}

impl Default for SpatialHash {
    fn default() -> SpatialHash {
        SpatialHash{
            hash: HashMap::new(),
            cell_length: 1.0,
        }
    }
}

impl SpatialHash {

    fn is_empty(&self) -> bool {
        self.hash.is_empty()
    }

    fn clear(&mut self) {
        self.hash.clear();
    }

    fn insert(&mut self, entity: Entity, collidable: Collidable, transform: Transform) {
        // Discretize all corners of the bounding box
        for corner in collidable.bounding_box.transformed(&transform) {
            // Add them to every unique GridCell
            let discretized_position = (
                (corner.x / self.cell_length).round() as u16,
                (corner.y / self.cell_length).round() as u16,
                (corner.z / self.cell_length).round() as u16,
            );
            if let Some(set) = self.hash.get_mut(&discretized_position) {
                set.insert(entity, (collidable.clone(), transform));
            } else {
                let mut map = HashMap::new();
                map.insert(entity, (collidable.clone(), transform));
                self.hash.insert(discretized_position, map);
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct Collidable {
   bounding_box: BoundingBox,
   collides_with: Vec<Entity> 
}

impl Collidable {
    pub fn new(bounding_box: BoundingBox) -> Collidable {
        Collidable {
            bounding_box,
            collides_with: Vec::new(),
        }
    }

    pub fn collides(&self) -> bool {
        self.collides_with.len() != 0
    }
}

fn _update_spatial_hash(
    mut spatial_hash: ResMut<SpatialHash>,
    mut query: Query<(Entity, &mut Collidable, &Transform), Changed<Transform>>,
) {
    // Try to update the hash to avoid rebuilding
    // I'm pretty sure the naive removal process makes this slower than rebuilding
    // Probably doesn't matter for now
    for (entity, mut collidable, transform) in query.iter_mut() {
        // Remove all occurences of entity from the mesh
        // How to do this smarter?
        for (_, map) in spatial_hash.hash.iter_mut() {
            map.remove(&entity);
        }

        // Past collision might not be relevant any more
        collidable.collides_with.clear();

        spatial_hash.insert(entity, collidable.clone(), transform.clone());
    }
}

fn rebuild_spatial_hash(
    mut spatial_hash: ResMut<SpatialHash>,
    mut query: Query<(Entity, &mut Collidable, &Transform)>,
) {
    spatial_hash.clear();
    for (entity, mut collidable, transform) in query.iter_mut() {
        collidable.collides_with.clear();
        // Add entities to the Hash, maybe I can get rid of the clone?
        spatial_hash.insert(entity, collidable.clone(), transform.clone());
    }
    assert!(!spatial_hash.is_empty());
}

fn collision_detection(
    mut spatial_hash: ResMut<SpatialHash>,
    // TODO I think I could reduce this to only entities whos transform has changed
    mut query: Query<(Entity, &mut Collidable, &Transform)>,
) {
    // For each cell
    for map in spatial_hash.hash.values_mut() {
        assert!(!map.is_empty());
        if map.len() < 2 {
            continue;
        }
        // I think I need this copy to iterate over the entities with defined order
        let keys: Vec<Entity> = map.keys().map(|e| e.clone()).collect();
        let length = keys.len();
        assert!(length >= 2);
        // Compare elements in the cell with each other
        for (i, entity) in keys.iter().enumerate() {
            let (collidable, transform) = map.get_mut(entity).unwrap();
            let bb = collidable.bounding_box.transformed(transform);
            for j in i+1..length {
                let other_entity = keys[j];
                let (other_collidable, other_transform) = map.get_mut(&other_entity).unwrap();
                let other_bb = other_collidable.bounding_box.transformed(&other_transform);
                let intersects = intersects(&bb, &other_bb);
                if intersects {
                    // Mark it in the collidable
                    other_collidable.collides_with.push(*entity);
                }
                // println!("Comparing boxes: {:?} and {:?} -> Collision: {}", bb, other_bb, intersects);
            }
        }
        // Write collisions back. I'm not sure if I should use a HashSet<Entity> for collides_with, because right now entities can be in there multiple times
        for (e, (local_collidable, _)) in map.iter() {
            if let Ok(mut collidable) = query.get_component_mut::<Collidable>(*e) {
                for colliding_entity in local_collidable.collides_with.iter() {
                    collidable.collides_with.push(*colliding_entity);
                }
            }
        }
    }
}

fn test_color_according_to_collision(
    mut query: Query<(&Collidable, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let green_material = materials.add(Color::GREEN.into());
    let red_material = materials.add(Color::RED.into());

    for (collidable, mut material_handle) in query.iter_mut() {
        if collidable.collides() {
            *material_handle = red_material.clone();
        } else {
            *material_handle = green_material.clone();
        }
    }
}

fn test_spawn_colliding_bodies(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
    let green_material = materials.add(Color::GREEN.into());

    let mut transforms: Vec<Transform> = Vec::new();
    transforms.push(Transform::from_translation(Vec3::new(1.0, 1.0, 1.0)));
    transforms.push(Transform::from_translation(Vec3::new(1.5, 1.5, 1.0)));
    transforms.push(Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)));
    transforms.push(Transform::from_translation(Vec3::new(1.0, 3.0, 1.0)));
    transforms.push(Transform::from_translation(Vec3::new(3.0, 2.0, 1.0)));
    
    for t in transforms.iter() {
        commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: green_material.clone(),
            transform: t.clone(),
            ..Default::default()
        })
        .with(Collidable::new(
            BoundingBox::default()
        ));
    }
}

#[test]
fn test_intersects_identical_boxes() {
    let box_1 = BoundingBox::default();
    assert!(intersects(&box_1, &box_1))
}

#[test]
fn test_intersects_separate_boxes_1() {
    let box_1 = BoundingBox::default();
    let box_2 = BoundingBox::new(
        Vec3::splat(0.0),
        Vec3::splat(1.0)
    );
    assert!(intersects(&box_1, &box_2))
}

#[test]
fn test_intersects_separate_boxes_2() {
    let box_1 = BoundingBox::default();
    let box_2 = BoundingBox::new(
        Vec3::splat(3.0),
        Vec3::splat(4.0)
    );
    assert!(!intersects(&box_1, &box_2))
}

