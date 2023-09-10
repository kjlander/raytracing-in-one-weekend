use crate::hittable::HitRecord;
use crate::random_f64;
use crate::ray::Ray;
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Color};
use std::fmt::Display;

/// Describes a material with Dielectric properties such as glass.
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

/// Describes a material with Lambertian reflectance.
pub struct Lambertian {
    pub albedo: Color,
}

/// Describes a material with mirror-like reflectance.
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

// Required Display for debugging purposes.
pub trait Material: std::fmt::Display {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::build(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_f64() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        *scattered = Ray::build(rec.p, direction);
        true
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::build(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal);
        *scattered = Ray::build(rec.p, reflected + self.fuzz * random_unit_vector());
        *attenuation = self.albedo;
        dot(&scattered.direction(), &rec.normal) > 0.
    }
}

impl Display for Dielectric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ir)
    }
}

impl Display for Lambertian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.albedo.e[0], self.albedo.e[1], self.albedo.e[2]
        )
    }
}

impl Display for Metal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.albedo.e[0], self.albedo.e[1], self.albedo.e[2]
        )
    }
}

impl Dielectric {
    /// Builds a new Dielectric from an ir value.
    pub fn build(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    /// Use Schlick's approximation for reflectance.
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Lambertian {
    /// Builds a new Lambertian from RGB values.
    pub fn build(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::build(r, g, b),
        }
    }

    /// Returns the albedo of the Lambertian.
    pub fn color(&self) -> Color {
        self.albedo
    }

    /// Constructs a new Lambertian fron an existing Color.
    pub fn from(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    /// Builds a new Metal from RGB and fuzz values.
    pub fn build(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Self {
            albedo: Color::build(r, g, b),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    /// Returns the albedo of the Metal.
    pub fn color(&self) -> Color {
        self.albedo
    }

    /// Constructs a new Metal from an existing Color and a fuzz value.
    pub fn from(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}
