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
        let d_x = (self.min.x - point.x).abs().min((point.x - self.max.x).abs());
        let d_y = (self.min.y - point.y).abs().min((point.y - self.max.y).abs());
        let d_z = (self.min.z - point.z).abs().min((point.z - self.max.z).abs());
        let distance =  (d_x.powi(2) + d_y.powi(2) + d_z.powi(2)).sqrt();
        if self.contains(point) { 
            return -distance; 
        }
        distance
    }
}

#[derive(Debug)]
pub struct BVHNode<T: Clone> {
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
                assert!(false);
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
                    bbox: AABB::outer(&left.bbox, &right.bbox),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right))
                })
             }
        }
    }

    pub fn get_closest(&self, position: &Vec3) -> Option<(T, AABB)> {
        // closest geometric distance to bounding box surface
        // If position is within 2 bounding boxes, the element with 
        // "further in" will be returned
        if !self.bbox.contains(&position){
            return None;
        }

        let mut left_contains = false;
        if let Some(left) = &self.left {
            left_contains = left.bbox.contains(&position);
        }
        let mut right_contains = false;
        if let Some(right) = &self.right {
            right_contains = right.bbox.contains(&position);
        }

        if left_contains && right_contains {
            let left_closest = self.left.as_ref().unwrap().get_closest(position).unwrap();
            let right_closest = self.right.as_ref().unwrap().get_closest(position).unwrap();

            let left_dist = left_closest.1.distance(position);
            let right_dist = right_closest.1.distance(position);
            if left_dist < right_dist { 
                return Some(left_closest);
            }
            else {
                return Some(right_closest);
            }
        } else if left_contains {
            return self.left.as_ref().unwrap().get_closest(position);
        } else if right_contains {
            return self.right.as_ref().unwrap().get_closest(position);
        } else {
            // I must be a leaf
            assert!(self.data.is_some());
            Some((self.data.as_ref().unwrap().clone(), self.bbox.clone()))
        }
    }

    pub fn get_n_closest(&self, position: &Vec3, n: i32) -> Option<Vec<T>> {
        None
    }

    pub fn get_in_radius (&self, position: &Vec3, radius: f32) -> Option<Vec<T>> {
        None
    }
}

fn test_construct_linear_boxes(n: i32) -> Vec<(i32, AABB)> {
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
    let data_and_boxes = test_construct_linear_boxes(5);
    let root = BVHNode::create(data_and_boxes);
    assert!(root.is_some());
}

// #[test]
// fn test_get_closest() {
//     let data_and_boxes = test_construct_linear_boxes(5);
//     let root = BVHNode::create(data_and_boxes);
//     assert!(root.is_some());

//     assert!()
// }

// Returns the first index thats part of the second section
fn split_heuristic<T: Clone>(mut data_and_boxes: Vec<(T, AABB)>) 
    -> (Vec<(T, AABB)>, Vec<(T, AABB)>) 
{
    assert!(data_and_boxes.len() > 1);

    let outer_box = data_and_boxes.iter().fold(
        data_and_boxes.get(0).unwrap().1, 
        |outer, current| {
        AABB::outer(&outer, &current.1)
    });
    // Find partition Axis (x, y or z)
    // Choose the one, where the outer centers have the largest distance 
    let outer_box_dimensions: Vec3 = outer_box.max - outer_box.min;

    // Split in the geometric middle of the axis with largest spread
    let mut before_split = Vec::new();
    let mut after_split = Vec::new();
    let max_dimension = outer_box_dimensions.max_element();
    if outer_box_dimensions.x == max_dimension { 
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
    else if outer_box_dimensions.y == max_dimension { 
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
