use crate::hittable::*;
use crate::ray::*;
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    /// Determines if a Ray hits anything when cast into the world (HittableList).
    fn hit(&self, r: &Ray, ray_t: RangeInclusive<f64>, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = *ray_t.end();

        for object in &self.objects {
            let interval = *ray_t.start()..=closest_so_far;
            if object.hit(r, interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.mat = temp_rec.mat.clone();
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }
        hit_anything
    }
}

/// For debugging. Prints the r, g, b, values for the albedo of each object within
/// the list.
impl Display for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = String::new();
        for object in &self.objects {
            x.push_str(format!("{}\n", object).as_str())
        }
        write!(f, "{}", x)
    }
}

impl HittableList {
    /// Adds a Hittable object to the HittableList.
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Rc::new(object));
    }

    /// Constructs a default-initialie HittableList.
    pub fn new() -> Self {
        Self::default()
    }
}
