use std::assert_eq;

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

    fn outer(a: &AABB, b: &AABB) -> AABB {
        AABB::new(
            a.min.min(b.min),
            a.max.max(b.max),
        )
    }

    fn contains(&self, point: &Vec3) -> bool {
        // check if elementwise min/max operations return
        // self.min/max
        let min = point.min(self.min);
        if min != self.min { return false; }
        
        let max = point.max(self.max);
        if max != self.max { return false; }
        else { return true; }
    }


    fn distance(&self, point: &Vec3) -> f32 {
        // Returns negative values if point is within bounding box
        if self.contains(point) { 
            let d_x = (point.x - self.min.x).min(self.max.x - point.x);
            let d_y = (point.y - self.min.y).min(self.max.y - point.y);
            let d_z = (point.z - self.min.z).min(self.max.z - point.z);
            return -1.0 * d_x.min(d_y).min(d_z); 
        } else {
            let d_x = (self.min.x - point.x).max(0.0).max(point.x - self.max.x);
            let d_y = (self.min.y - point.y).abs().min((point.y - self.max.y).abs());
            let d_z = (self.min.z - point.z).abs().min((point.z - self.max.z).abs());
            return  (d_x.powi(2) + d_y.powi(2) + d_z.powi(2)).sqrt();
        }
    }
    
    pub fn translated(&self, translation: &Vec3) -> AABB {
        AABB::new(self.min + *translation, self.max + *translation)
    }
}

#[test]
fn test_aabb_distance() {
    let bbox = AABB::new(Vec3::splat(1.0), Vec3::splat(2.0));
    assert_eq!(bbox.distance(&Vec3::new(0.0, 1.0, 1.0)), 1.0);
    assert_eq!(bbox.distance(&Vec3::new(1.0, 0.0, 1.0)), 1.0);
    assert_eq!(bbox.distance(&Vec3::new(1.0, 1.0, 0.0)), 1.0);
    assert_eq!(bbox.distance(&Vec3::splat(1.0)), 0.0);
    assert_eq!(bbox.distance(&Vec3::splat(2.0)), 0.0);
    assert_eq!(bbox.distance(&Vec3::splat(1.5)), -0.5);
    assert_eq!(bbox.distance(&Vec3::splat(1.25)), -0.25);
}

#[derive(Debug)]
pub struct BVHNode<T: Clone> {
    data: Option<T>,
    bbox: AABB,
    left: Option<Box<BVHNode<T>>>,
    right: Option<Box<BVHNode<T>>>,
}

// static mut CALLS_TO_CREATE: i32 = 0;

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
        // unsafe {
        //     CALLS_TO_CREATE += 1;
        //     println!("create was called: {} times", CALLS_TO_CREATE);
        // }
        match data_and_boxes.len() {
            0 => { 
                // This should not happen
                assert!(false);
                return None;
            }
            1 => { 
                // Become Leaf node
                let data_and_box = data_and_boxes.pop().unwrap();
                return Some(BVHNode::new(data_and_box.0, data_and_box.1));
            }
            _ => { 
                // Defer to children and set their combined BoundingBox as yours
                let partitions = split_heuristic(data_and_boxes);
                // TODO I should get rid of those copies
                let left = BVHNode::create(partitions.0).unwrap();
                let right = BVHNode::create(partitions.1).unwrap();

                return Some(BVHNode{
                    data: None,
                    bbox: AABB::outer(&left.bbox, &right.bbox),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right))
                });
             }
        }
    }

    fn is_leaf(&self) -> bool {
        if self.left.is_none() && self.right.is_none() {
            assert!(self.data.is_some());
            return true;
        } else {
            assert!(self.data.is_none());
            return false;
        }
    }

    pub fn get_closest(&self, position: &Vec3) -> Option<(T, AABB)> {
        // closest geometric distance to bounding box surface
        // If position is within 2 bounding boxes, the element
        // "further in" will be returned
        if self.is_leaf() {
            return Some((self.data.as_ref().unwrap().clone(), self.bbox.clone()));
        }

        let mut left_contains = false;
        if let Some(left) = &self.left {
            left_contains = left.bbox.contains(&position);
        } else {
            // Only right is some
            return self.right.as_ref().unwrap().get_closest(position);
        }
        let mut right_contains = false;
        if let Some(right) = &self.right {
            right_contains = right.bbox.contains(&position);
        } else {
            // Only left is some
            return self.left.as_ref().unwrap().get_closest(position);
        }
        // Both right and left are some
        let left_closest;
        let right_closest;
        if left_contains && !right_contains{
            // I can potentially prune the right branch
            left_closest = self.left.as_ref().unwrap().get_closest(position).unwrap();
            let dist = left_closest.1.distance(position);
            if dist < self.right.as_ref().unwrap().bbox.distance(position) {
                return Some(left_closest);
            } else {
                right_closest = self.left.as_ref().unwrap().get_closest(position).unwrap();
            }
        } 
        else if right_contains && !left_contains {
            // I can potentially prune the left branch
            right_closest = self.left.as_ref().unwrap().get_closest(position).unwrap();
            let dist = right_closest.1.distance(position);
            if dist < self.left.as_ref().unwrap().bbox.distance(position) {
                return Some(right_closest);
            } else {
                left_closest = self.left.as_ref().unwrap().get_closest(position).unwrap();
            }
        } 
        else {
            // TODO I can potentially prune if I get lucky and the same condition
            // As in the cases above is true
            left_closest = self.left.as_ref().unwrap().get_closest(position).unwrap();
            right_closest = self.right.as_ref().unwrap().get_closest(position).unwrap();
        } 
        // Pick the closer one
        let left_dist = left_closest.1.distance(position);
        let right_dist = right_closest.1.distance(position);
        if left_dist < right_dist { 
            return Some(left_closest);
        }
        else {
            return Some(right_closest);
        }
    }

    pub fn get_in_radius (&self, position: &Vec3, radius: f32) -> Option<Vec<T>> {
        if self.bbox.distance(position) > radius {
            return None;
        } 
        if self.is_leaf() {
            return Some(vec![self.data.as_ref().unwrap().clone()]);
        }
        let mut return_data = Vec::new();
        if let Some(left) = &self.left {
            if let Some(mut data) = left.get_in_radius(position, radius) {
                return_data.append(&mut data);
            }
        }
        if let Some(right) = &self.right {
            if let Some(mut data) = right.get_in_radius(position, radius) {
                return_data.append(&mut data);
            }
        }
        Some(return_data)
    }
    
    // pub fn get_n_closest(&self, position: &Vec3, n: i32) -> Option<Vec<T>> {
    //     None
    // }
}

fn _test_construct_linear_boxes(n: i32) -> Vec<(i32, AABB)> {
    let mut data_and_boxes = Vec::new();
    for i in 0..n {
        data_and_boxes.push((i, AABB::new(
            Vec3::splat(i as f32 * 2.0), 
            Vec3::splat(i as f32 * 2.0 + 1.0 ))));
    }
    data_and_boxes
}

#[test]
fn test_bvh_create() {
    let data_and_boxes = _test_construct_linear_boxes(5);
    let root = BVHNode::create(data_and_boxes);
    assert!(root.is_some());
}

#[test]
fn test_get_closest() {
    let data_and_boxes = _test_construct_linear_boxes(5);
    let root_opt = BVHNode::create(data_and_boxes);
    assert!(root_opt.is_some());
    let root = root_opt.unwrap();
    // on the border
    let closest_0 = root.get_closest(&Vec3::splat(0.5));
    assert!(closest_0.is_some());
    assert_eq!(closest_0.unwrap().0, 0);
    // in the middle
    let closest_1 = root.get_closest(&Vec3::splat(1.5));
    assert!(closest_1.is_some());
    assert_eq!(closest_1.unwrap().0, 1);
    // outside
    let closest_4 = root.get_closest(&Vec3::splat(20.0));
    assert!(closest_4.is_some());
    assert_eq!(closest_4.unwrap().0, 4);
}

// Returns the first index thats part of the second section
fn split_heuristic<T: Clone>(mut data_and_boxes: Vec<(T, AABB)>) 
    -> (Vec<(T, AABB)>, Vec<(T, AABB)>) 
{
    assert!(data_and_boxes.len() > 1);

    let outer_box = data_and_boxes
        .iter()
        .fold(
            data_and_boxes.get(0).unwrap().1, 
            |outer, current| 
            {
                AABB::outer(&outer, &current.1)
            }
        );
    
    let mut before_split = Vec::new();
    let mut after_split = Vec::new();
    // Find partition Axis (x, y or z)
    // Choose the one, where the outer centers have the largest distance 
    let outer_box_dimensions: Vec3 = outer_box.max - outer_box.min;
    let max_dimension = outer_box_dimensions.max_element();
    // Split in the geometric middle of the axis with largest spread
    if outer_box_dimensions.x == max_dimension { 
        let center = outer_box.min.x + (outer_box_dimensions/2.0).x;
        data_and_boxes.sort_by(|a, b| { 
            assert!(b.1.center.x.is_finite());
            a.1.center.x.partial_cmp(&b.1.center.x).unwrap() 
        });
        for p in data_and_boxes.iter() {
            if p.1.center.x <= center {
                // TODO I should prevent the copies of data Members by 
                before_split.push(p.clone());
            } else {
                after_split.push(p.clone());
            }
        }
    }
    else if outer_box_dimensions.y == max_dimension { 
        let center = outer_box.min.y + (outer_box_dimensions/2.0).y;
        data_and_boxes.sort_by(|a, b| { 
            assert!(b.1.center.y.is_finite());
            a.1.center.y.partial_cmp(&b.1.center.y).unwrap() 
        });
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
        assert_eq!(outer_box_dimensions.z, max_dimension);
        let center = outer_box.min.z + (outer_box_dimensions/2.0).z;
        data_and_boxes.sort_by(|a, b| { 
            assert!(b.1.center.z.is_finite());
            a.1.center.z.partial_cmp(&b.1.center.z).unwrap() });
        for p in data_and_boxes.iter() {
            if p.1.center.z <= center {
                // TODO I should prevent the copies of data Members by 
                before_split.push(p.clone());
            } else {
                after_split.push(p.clone());
            }
        }
    }

    // println!("elements before|after split: {}|{}", before_split.len(), after_split.len());
    assert!(before_split.len() > 0);
    assert!(after_split.len() > 0);
    assert_eq!(before_split.len() + after_split.len(), data_and_boxes.len());
    (before_split, after_split)
}


#[test]
fn test_aabb_combine() {
    let a = AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0,3.0,3.0));
    let b = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(2.0,2.0,2.0));
    let c = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(3.0,3.0,3.0));
    assert_eq!(AABB::outer(&a, &b), c);
}
#[test]
fn test_aabb_contains() {
    let a = AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0,3.0,3.0));
    assert!(a.contains(&Vec3::splat(2.0)));
    assert!(!a.contains(&Vec3::splat(4.0)));
}
