extern crate nalgebra_glm as glm;
pub mod camera;
pub mod geometry;

use crate::camera::Camera;
use crate::geometry::hittable::{HittableList, HittableObject, Sphere};

fn output_file(data: Vec<u8>, width: u32, height: u32) {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new(r"out/out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    // let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(&data).unwrap(); // Save
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    println!("Hello, world!");

    let world = HittableObject::HittableListObject(HittableList {
        objects: vec![
            HittableObject::SphereObject(Sphere {
                center: glm::vec3(-0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            HittableObject::SphereObject(Sphere {
                center: glm::vec3(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    });
    let camera: Camera = Camera::new(aspect_ratio, image_width);
    let data = camera.render(&world);
    output_file(data, camera.image_width as u32, camera.image_height as u32);
}
