use crate::vec3::{Point3, Vec3};

/// Describes a ray by its origin and direction.
#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    /// Returns the 3D point at location t along the Ray.
    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    /// Builds a new Ray.
    pub fn build(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// Returns the direction the Ray points.
    pub fn direction(self) -> Vec3 {
        self.dir
    }

    /// Constructs a default-initialized Ray.
    pub fn new() -> Self {
        Self::default()
    }
}
