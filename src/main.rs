use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::material::{Dielectric, Lambertian, Metal};
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::{random_f64, random_f64_in, vec3::*};
use std::rc::Rc;

fn main() {
    // World

    let mut world = HittableList::new();

    // Image 18 - With Hollow Glass Sphere
    /*
    let material_ground = Rc::new(Lambertian::build(0.8, 0.8, 0.0));
    let material_center = Rc::new(Lambertian::build(0.1, 0.2, 0.5));
    let material_left = Rc::new(Dielectric::build(1.5));
    let material_right = Rc::new(Metal::build(0.8, 0.6, 0.2, 0.0));

    world.add(Sphere::build(
        Point3::build(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::build(
        Point3::build(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add(Sphere::build(
        Point3::build(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    world.add(Sphere::build(
        Point3::build(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    ));
    world.add(Sphere::build(
        Point3::build(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
    */

    // Image 19 - Testing Camera FOV
    /*
    let R: f64 = (std::f64::consts::PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::build(0.0, 0.0, 1.0));
    let material_right = Rc::new(Lambertian::build(1.0, 0.0, 0.0));

    world.add(Sphere::build(Point3::build(-R, 0.0, -1.0), R, material_left));
    world.add(Sphere::build(Point3::build(R, 0.0, -1.0), R, material_right));
    */

    // Book Cover Image
    let ground_material = Rc::new(Lambertian::build(0.5, 0.5, 0.5));
    world.add(Sphere::build(
        Point3::build(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::build(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::build(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = random_vec3() * random_vec3();
                    let sphere_material = Rc::new(Lambertian::from(albedo));
                    world.add(Sphere::build(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Color = random_vec3_in_range(0.5, 1.0);
                    let fuzz = random_f64_in(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::from(albedo, fuzz));
                    world.add(Sphere::build(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::build(1.5));
                    world.add(Sphere::build(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::build(1.5));
    world.add(Sphere::build(Point3::build(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Rc::new(Lambertian::build(0.4, 0.2, 0.1));
    world.add(Sphere::build(Point3::build(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Rc::new(Metal::build(0.7, 0.6, 0.5, 0.0));
    world.add(Sphere::build(Point3::build(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Point3::build(13.0, 2.0, 3.0);
    cam.look_at = Point3::build(0.0, 0.0, 0.0);
    cam.vup = Vec3::build(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
