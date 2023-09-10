use crate::{random_f64, random_f64_in};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

// Type aliases for Vec3
pub type Point3 = Vec3; // 3D point
pub type Color = Vec3; // RGB color

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0., 0., 0.] }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2]
    }
}

///////////////////////////////////////////////////////////////////////////////
// Operators

// += operator
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

// /= operator for f64 rhs
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs
    }
}

// *= operator for f64 rhs
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

// [] operator
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// Unary - operator
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// Unary - operator for &Vec3
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// + operator
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

// - operator
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

// Vec3 * Vec3 operator
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

// f64 * Vec3 operator
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

// Vec3 * f64 operator
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

// Vec3 * f64 operator
impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs as f64,
                self.e[1] * rhs as f64,
                self.e[2] * rhs as f64,
            ],
        }
    }
}

// Vec3 / f64 operator
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Vec3 {
    /// Builds a new Vec3 from x, y, and z components.
    pub const fn build(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    /// Returns the length of a Vec3.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the squared lenght of a Vec3.
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Returns true if the Vec3 is very close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

    /// Constructs a default initlaized Vec3, with all 3 components set to 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reutrns the x component of the Vec3.
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    /// Reutrns the y component of the Vec3.
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    /// Reutrns the z component of the Vec3.
    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

/// Returns the cross product of two Vec3s.
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
            lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
            lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
        ],
    }
}

/// Returns the dot product of two Vec3s.
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
}

/// Converts from linear space to gamma space by taking the square root
/// of the linear component.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

/// Returns a random Vec3 with x, y, and z in the range 0..=1.
pub fn random_vec3() -> Vec3 {
    Vec3 {
        e: [random_f64(), random_f64(), random_f64()],
    }
}

/// Returns a random Vec3 in a hemisphere with no dependance on angle
/// from the normal.
pub fn random_vec3_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_vec3_in_unit_sphere();
    if dot(&on_unit_sphere, normal) > 0. {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

/// Returns a random Vec3 with x, y, and z in the range specified.
pub fn random_vec3_in_range(min: f64, max: f64) -> Vec3 {
    Vec3 {
        e: [
            random_f64_in(min, max),
            random_f64_in(min, max),
            random_f64_in(min, max),
        ],
    }
}

/// Returns a random Vec3 within a disk of unit length.
pub fn random_vec3_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::build(random_f64_in(-1.0, 1.0), random_f64_in(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Returns a random Vec3 within a unit sphere.
pub fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_in_range(-1., 1.);
        if p.length_squared() >= 1. {
            continue;
        } else {
            return p;
        }
    }
}

/// Return a random Vec3 on the surface of a unit sphere.
/// For true Lambertian reflection.
pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_vec3_in_unit_sphere())
}

/// Returns the reflected ray direction for mirror-like surfaces, like smooth
/// metals.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(v, n) * *n
}

/// Reutrns the vector that results from refraction.
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
    r_out_perp + r_out_parallel
}

/// Return the unit vector of a Vec3.
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

/// For writing colors to a PPM file, returns RGB as a space-separates String.
pub fn write_color(pixel_color: &Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1. / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Apply the linear to gamma transform.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Write the translated [0,255] value of each color component.
    let intensity = 0.000..0.999;
    format!(
        "{} {} {}",
        (256.0 * r.clamp(intensity.start, intensity.end)) as i32,
        (256.0 * g.clamp(intensity.start, intensity.end)) as i32,
        (256.0 * b.clamp(intensity.start, intensity.end)) as i32,
    )
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cross() {
        let x = Vec3::build(1.0, 0.0, 0.0);
        let y = Vec3::build(0.0, 1.0, 0.0);
        let z = Vec3::build(0.0, 0.0, 1.0);
        assert_eq!(cross(&x, &y), z);
    }
}
