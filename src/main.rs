extern crate nalgebra_glm as glm;
extern crate rand;
pub mod camera;
pub mod geometry;
pub mod material;

use std::fs::File;

use material::Dielectric;

use crate::{
    camera::Camera,
    geometry::hittable::{HittableList, HittableObject, Sphere},
    material::{Lambertian, MaterialObject, Metal},
};

type Vec3 = glm::TVec3<f64>;

fn output_file(data: Vec<u8>, width: u32, height: u32) {
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new(r"out/out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0));
    // let source_chromaticities: png::SourceChromaticities = png::SourceChromaticities::new(
    //     // Using unscaled instantiation here
    //     (0.31270, 0.32900),
    //     (0.64000, 0.33000),
    //     (0.30000, 0.60000),
    //     (0.15000, 0.06000),
    // );
    // encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    // let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(&data).unwrap(); // Save
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 10;

    let world = HittableObject::HittableListObject(HittableList {
        objects: vec![
            HittableObject::SphereObject(Sphere::new(
                Vec3::new(0.0, 0.0, -1.2),
                0.5,
                MaterialObject::LambertianObject(Lambertian {
                    albedo: Vec3::new(0.1, 0.2, 0.5),
                }),
            )),
            HittableObject::SphereObject(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                MaterialObject::LambertianObject(Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                }),
            )),
            HittableObject::SphereObject(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                MaterialObject::DielectricObject(Dielectric {
                    refraction_index: 1.5,
                }),
            )),
            HittableObject::SphereObject(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.4,
                MaterialObject::DielectricObject(Dielectric {
                    refraction_index: 1.0 / 1.5,
                }),
            )),
            HittableObject::SphereObject(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                MaterialObject::MetalObject(Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzz: 1.0,
                }),
            )),
        ],
    });

    let now = std::time::Instant::now();
    let camera: Camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    let data = camera.render(&world);
    println!("render time: {}", now.elapsed().as_millis());
    output_file(data, camera.image_width as u32, camera.image_height as u32);
}
