use crate::hittable::{HitRecord, Hittable};
use crate::random_f64;
use crate::ray::Ray;
use crate::vec3::{cross, random_vec3_in_unit_disk, unit_vector, write_color, Color, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel.
    pub max_depth: i32,         // Maximum number of ray bounces into scene.
    pub look_from: Point3,      // Point camera is looking from
    pub look_at: Point3,        // Point camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction
    pub vfov: f64,              // Vertical view angle (field of view)
    pub defocus_angle: f64,     // Variation angle of rays through each pixel
    pub focus_dist: f64,        // Distance from camera look_from point to plane of perfect focus
    image_height: i32,          // Rendered image height
    center: Point3,             // Camera center
    pixel100_loc: Point3,       // Location of pixel 0, 0
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
    u: Vec3,                    // Camera frame basis vectors
    v: Vec3,                    //
    w: Vec3,                    //
    defocus_disk_u: Vec3,       // Defocus disk horizontal radius
    defocus_disk_v: Vec3,       // Defocus disk vertical radius
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 1.0;
        let image_width = 100;
        let image_height = 100;
        let samples_per_pixel = 10;
        let max_depth = 10;
        let vfov = 90.0_f64;
        let look_from = Point3::build(0.0, 0.0, -1.0);
        let look_at = Point3::new();
        let center = look_from;
        let vup = Vec3::build(0.0, 1.0, 0.0);
        let defocus_angle = 0.0;
        let focus_dist = 10.0;

        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u, v, w unit basis vectors fro the camera coordinate frame.
        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::build(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::build(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (((defocus_angle / 2.0) as f64).to_radians()).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio,
            image_height,
            image_width,
            samples_per_pixel,
            max_depth,
            look_from,
            look_at,
            vup,
            vfov,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl Camera {
    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_vec3_in_unit_disk();
        self.center + (p.e[0] * self.defocus_disk_u) + (p.e[1] * self.defocus_disk_v)
    }

    /// Get a randomly-sampled camera ray for the pixel at location i,j, originating from
    /// the camera defocus disk.
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel100_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            orig: ray_origin,
            dir: ray_direction,
        }
    }

    /// Calculates and sets the derived fields of the Camera struct.
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.look_from;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors fro the camera coordinate frame.
        self.w = unit_vector(&(self.look_from - self.look_at));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel100_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius =
            self.focus_dist * (((self.defocus_angle / 2.0) as f64).to_radians()).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Returns a default-initialized camera.
    pub fn new() -> Camera {
        Camera::default()
    }

    /// Returns a random point in the square surrounging a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    /// Renders the output image.
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {} \n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                println!("{}", write_color(&pixel_color, self.samples_per_pixel));
            }
        }
        eprintln!("\rDone.\n");
    }

    /// Determines the color returned by a Ray when cast into the world.
    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();

        //If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new();
        }

        if world.hit(r, 0.001..=f64::INFINITY, &mut rec) {
            let mut scattered: Ray = Ray::new();
            let mut attenuation: Color = Color::new();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Camera::ray_color(&scattered, depth - 1, world);
            }
            return Color::new();
        }

        let unit_direction = unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::build(1.0, 1.0, 1.0) + a * Color::build(0.5, 0.7, 1.0)
    }
}
