use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
    center: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB {
            min,
            max,
            center: (min + max) / 2.0
        }
    }

    fn combine(a: &AABB, b: &AABB) -> AABB {
        let min = Vec3 {
            x: if a.min.x < b.min.x { a.min.x } else { b.min.x },
            y: if a.min.y < b.min.y { a.min.y } else { b.min.y },
            z: if a.min.z < b.min.z { a.min.z } else { b.min.z },
        };
        let max = Vec3 {
            x: if a.max.x > b.max.x { a.max.x } else { b.max.x },
            y: if a.max.y > b.max.y { a.max.y } else { b.max.y },
            z: if a.max.z > b.max.z { a.max.z } else { b.max.z },
        };
        AABB::new(
            min,
            max,
        )
    }
}

impl BVHPrimitive for AABB {
    fn get_bounding_box(&self) -> AABB {
        *self
    }
}

#[test]
fn test_aabb_combine() {
    let a = AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0,3.0,3.0));
    let b = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(2.0,2.0,2.0));
    let c = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(3.0,3.0,3.0));
    assert_eq!(AABB::combine(&a, &b), c);
}

pub trait BVHPrimitive {
    fn get_bounding_box(&self) -> AABB;
}

struct BVHNode<T: BVHPrimitive> {
    data: Option<T>,
    bbox: AABB,
    left: Option<Box<BVHNode<T>>>,
    right: Option<Box<BVHNode<T>>>,
}

impl<T> BVHNode<T>
where T: BVHPrimitive + Clone {
    fn new(data: T) -> BVHNode<T> {
        BVHNode {
            data: Some(data.clone()),
            bbox: data.get_bounding_box(),
            left: None,
            right: None,
        }
    }
    pub fn create(mut primitives: Vec<T>) -> Option<BVHNode<T>> {
        match primitives.len() {
            0 => { return None }
            1 => { 
                Some(BVHNode::new(primitives.pop().unwrap())) 
            }
            2 => { 
                let a = primitives.pop().unwrap();
                let b = primitives.pop().unwrap();
                Some(BVHNode{
                    data: None,
                    bbox: AABB::combine(&a.get_bounding_box(), &b.get_bounding_box(),),
                    left: Some(Box::new(BVHNode::new(a))),
                    right: Some(Box::new(BVHNode::new(b))),
                }) 
            }
            _ => { 
                let split_idx = split_heuristic(&primitives);
                // TODO I should get rid of those copies
                let mut data_left = Vec::new();
                data_left.extend_from_slice(&primitives[..split_idx]);
                let left = BVHNode::create(data_left).unwrap();

                let mut data_right = Vec::new();
                data_right.extend_from_slice(&primitives[split_idx..]);
                let right = BVHNode::create(data_right).unwrap();

                Some(BVHNode{
                    data: None,
                    bbox: AABB::combine(&left.bbox, &right.bbox),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right))
                })
             }
        }
    }
}

fn split_heuristic<T: BVHPrimitive>(primitives: &Vec<T>) -> usize {
    0
}

pub struct BVH<T: BVHPrimitive> {
    beppo: Vec<T>
}

impl<T> BVH<T> 
where T: BVHPrimitive {
    pub fn new() -> BVH<T> {
        BVH{
            beppo: Vec::new()
        }
    }

    pub fn from_primitives(primitives: &Vec<T>) -> BVH<T> {
        let mut bbs = Vec::new();
        for p in primitives.iter() {
            bbs.push(p.get_bounding_box());
        }

        let length = bbs.len();
        let a = bbs.len() / 2;

        bbs.sort_by(|a, b| {
            a.center.x.partial_cmp(&b.center.x).unwrap() 
        });



        bbs.sort_by(|a, b| {
            a.center.y.partial_cmp(&b.center.y).unwrap() 
        });
        bbs.sort_by(|a, b| {
            a.center.z.partial_cmp(&b.center.z).unwrap() 
        });

        
        BVH::new()
    }
}
