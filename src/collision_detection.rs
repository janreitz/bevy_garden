use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpatialHash>()
            .add_system(collision_detection.system())
            .add_system(test_color_according_to_collision.system())
            .add_startup_system(test_spawn_colliding_bodies.system());
    }
}

#[derive(Clone, Copy)]
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
    fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox {
            min,
            max,
            current_corner: 0
        }
    }

    fn transformed(&self, transform: &Transform) -> BoundingBox {
        self.translated(&transform.translation).rotated(&transform.rotation)
        // Should I also scale the bounding box?
    }
    // Are these new allocations expensive?
    fn translated(&self, translation: &Vec3) -> BoundingBox {
        BoundingBox::new(self.min + *translation, self.max + *translation)
    }

    fn rotated(&self, rotation: &Quat) -> BoundingBox {
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
    hash: HashMap<(u16, u16, u16), HashSet<Entity>>,
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

    fn clear(&mut self) {
        self.hash.clear();
    }

    fn insert(&mut self, entity: Entity, bounding_box: BoundingBox) {
        // Discretize all corners of the bounding box
        for corner in bounding_box {
            // Add them to every unique GridCell
            let discretized_position = (
                (corner.x / self.cell_length).round() as u16,
                (corner.y / self.cell_length).round() as u16,
                (corner.z / self.cell_length).round() as u16,
            );
            if let Some(set) = self.hash.get_mut(&discretized_position) {
                set.insert(entity);
            } else {
                let mut hash_set = HashSet::new();
                hash_set.insert(entity);
                self.hash.insert(discretized_position, hash_set);
            }
        }
    }
}

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
        self.collides_with.len() == 0
    }
}

fn collision_detection(
    mut spatial_hash: ResMut<SpatialHash>,
    // TODO I think I could reduce this to only entities whos transform has changed
    mut query: Query<(Entity, &mut Collidable, &Transform)>,
) {
    // TODO do not rebuild the hash every iteration 
    spatial_hash.clear();
    for (entity, collidable, transform) in query.iter_mut() {
        // Add entities to the Hash
        spatial_hash.insert(entity, collidable.bounding_box.transformed(transform));
    }
    for set in spatial_hash.hash.values() {
        // Query and transform all bounding boxes
        let mut entities_and_boxes: Vec<(Entity, BoundingBox)>;
        // Compare every element with each other

        // let entities_and_boxes: Vec<> = set.iter()
        //     .filter_map(|e| {
        //         if let Ok(collidable) = query.get_component_mut::<Collidable>(*e){
        //             Some((e, collidable.bounding_box.clone()))
        //         } else {
        //             None
        //         }
        //     })
        //     .filter_map(|(e, b)| {
        //         if let Ok(transform) = query.get_component_mut::<Transform>(*e){
        //             Some((e.to_owned(), b.transformed(&transform)))
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
                 

        for entity in set.iter() {
            let mut bounding_box = BoundingBox::default();
            {
                if let Ok(collidable) = query.get_component_mut::<Collidable>(*entity){
                    bounding_box = collidable.bounding_box.clone();
                }
            }
            if let Ok(transform) = query.get_component_mut::<Transform>(*entity){
                let entity_and_box = (
                    entity.to_owned(), 
                    bounding_box.transformed(&transform),
                );
                entities_and_boxes.push(entity_and_box.to_owned());
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let monkey_handle: Handle<Mesh> = asset_server.load("models/basic_shapes/monkey.glb#Mesh0/Primitive0");
    let green_material = materials.add(Color::GREEN.into());
    let transform = Transform::from_translation(Vec3::splat(3.0));
    commands
    .spawn(PbrBundle {
        mesh: monkey_handle,
        material: green_material,
        transform: transform,
        ..Default::default()
    })
    .with(Collidable::new(
        BoundingBox::default()
    ));
}