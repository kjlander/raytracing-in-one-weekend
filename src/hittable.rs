use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{dot, Color, Point3, Vec3};

use std::fmt::Display;
use std::ops::RangeInclusive;
use std::rc::Rc;

/// Contains data related to a Ray hitting an object.
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable: Display {
    fn hit(&self, r: &Ray, ray_t: RangeInclusive<f64>, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    /// Sets the hit record normal vector.
    /// NOTE: the parameter 'outward normal' is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }

    /// Constructs a default-initialize HitRecord.
    /// Default HitRecords are generally not suitable for immediate use.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            mat: Rc::new(Lambertian {
                albedo: Color::new(),
            }),
            t: 0.,
            front_face: true,
        }
    }
}
