use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
        AABB::new(
            a.min.min(b.min),
            a.max.max(b.max),
        )
    }
}

struct BVHNode<T: Clone> {
    data: Option<T>,
    bbox: AABB,
    left: Option<Box<BVHNode<T>>>,
    right: Option<Box<BVHNode<T>>>,
}

impl<T> BVHNode<T>
where T: Clone {
    fn new(data: T, bbox: AABB) -> BVHNode<T> {
        BVHNode {
            data: Some(data),
            bbox: bbox,
            left: None,
            right: None,
        }
    }
    pub fn create(mut data_and_boxes: Vec<(T, AABB)>) -> Option<BVHNode<T>> {
        match data_and_boxes.len() {
            0 => { 
                // This should not happen
                None 
            }
            1 => { 
                // Become Leaf node
                let data_and_box = data_and_boxes.pop().unwrap();
                Some(BVHNode::new(data_and_box.0, data_and_box.1)) 
            }
            _ => { 
                // Defer to children and set their combined BoundingBox as yours
                let partitions = split_heuristic(data_and_boxes);
                // TODO I should get rid of those copies
                let left = BVHNode::create(partitions.0).unwrap();
                let right = BVHNode::create(partitions.1).unwrap();

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

// Returns the first index thats part of the second section
fn split_heuristic<T: Clone>(mut data_and_boxes: Vec<(T, AABB)>) 
    -> (Vec<(T, AABB)>, Vec<(T, AABB)>) 
{
    assert!(data_and_boxes.len() > 1);
    // Split in the geometric middle of the axis with largest spread

    let outer_box = data_and_boxes.iter().fold(
        data_and_boxes.get(0).unwrap().1, 
        |outer, current| {
        AABB::combine(&outer, &current.1)
    });
    // Find partition Axis (x, y or z)
    // Choose the one, where the outer centers have the largest distance 
    let outer_box_dimensions: Vec3 = outer_box.max - outer_box.min;

    let mut before_split = Vec::new();
    let mut after_split = Vec::new();

    let max = outer_box_dimensions.max_element();
    if outer_box_dimensions.x == max { 
        let center = outer_box.min.x + (outer_box_dimensions/2.0).x;
        data_and_boxes.sort_by(|a, b| { a.1.center.x.partial_cmp(&b.1.center.x).unwrap() });
        for p in data_and_boxes.iter() {
            if p.1.center.x <= center {
                // TODO I should prevent the copies of data Members by 
                before_split.push(p.clone());
            } else {
                after_split.push(p.clone());
            }
        }
    }
    if outer_box_dimensions.y == max { 
        let center = outer_box.min.y + (outer_box_dimensions/2.0).y;
        data_and_boxes.sort_by(|a, b| { a.1.center.y.partial_cmp(&b.1.center.y).unwrap() });
        for p in data_and_boxes.iter() {
            if p.1.center.y <= center {
                // TODO I should prevent the copies of data Members by 
                before_split.push(p.clone());
            } else {
                after_split.push(p.clone());
            }
        }
    }
    else { 
        let center = outer_box.min.z + (outer_box_dimensions/2.0).z;
        data_and_boxes.sort_by(|a, b| { a.1.center.z.partial_cmp(&b.1.center.z).unwrap() });
        for p in data_and_boxes.iter() {
            if p.1.center.z <= center {
                // TODO I should prevent the copies of data Members by 
                before_split.push(p.clone());
            } else {
                after_split.push(p.clone());
            }
        }
    }

    (before_split, after_split)
}


#[test]
fn test_aabb_combine() {
    let a = AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0,3.0,3.0));
    let b = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(2.0,2.0,2.0));
    let c = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(3.0,3.0,3.0));
    assert_eq!(AABB::combine(&a, &b), c);
}
