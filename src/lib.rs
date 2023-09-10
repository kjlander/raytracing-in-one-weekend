use rand::Rng;

// Modules

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

// Utility functions

// Returns a random f64 in the range 0.0..=1.0
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..=1.0)
}

/// Returns a random f64 in the given range.
pub fn random_f64_in(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
