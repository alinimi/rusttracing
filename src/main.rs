extern crate nalgebra_glm as glm;
use core::f32::INFINITY;

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

struct Ray {
    origin: glm::Vec3,
    direction: glm::Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> glm::Vec3 {
        self.origin + self.direction * t
    }
}

struct HitRecord {
    point: glm::Vec3,
    normal: glm::Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn new(point: glm::Vec3, t: f32, outward_normal: glm::Vec3, r: &Ray) -> Self {
        let front_face = outward_normal.dot(&r.direction) < 0.0;
        Self {
            point: point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t: t,
            front_face: front_face,
        }
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

struct Sphere {
    center: glm::Vec3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: glm::Vec3 = self.center - r.origin;
        let a = glm::length2(&r.direction);
        let h = glm::dot(&r.direction, &oc);
        let c = glm::length2(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        // let mut hitv: HitRecord = HitRecord{};
        let root0 = (h - f32::sqrt(discriminant)) / a;
        let root1 = (h + f32::sqrt(discriminant)) / a;

        let t: f32;

        if ray_t.surrounds(root0) {
            t = root0;
        } else if ray_t.surrounds(root1) {
            t = root1;
        } else {
            return None;
        }

        let point = r.at(t);
        let hit = HitRecord::new(point, t, (point - self.center).normalize(), r);
        Some(hit)
    }
}

enum HittableObject {
    SphereObject(Sphere),
    HittableListObject(HittableList),
}

impl Hittable for HittableObject {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::SphereObject(sphere) => sphere.hit(r, ray_t),
            Self::HittableListObject(hittable_list) => hittable_list.hit(r, ray_t),
        }
    }
}

fn ray_color(obj: &HittableObject, r: &Ray) -> glm::Vec3 {
    let unit_direction: glm::Vec3 = r.direction.normalize();

    let closest_hit: Option<HitRecord> = obj.hit(
        r,
        Interval {
            min: 0.0,
            max: INFINITY,
        },
    );

    if let Some(closest_final) = &closest_hit {
        return 0.5 * (closest_final.normal + glm::vec3(1.0, 1.0, 1.0));
    }
    let a = 0.5 * unit_direction.y + 0.5;
    glm::Vec3::new(0.5, 0.7, 1.0) * a + (1.0 - a) * glm::Vec3::new(1.0, 1.0, 1.0)
}

struct HittableList {
    objects: Vec<HittableObject>,
}

impl HittableList {
    fn add(&mut self, obj: HittableObject) {
        self.objects.push(obj);
    }
}

#[derive(Copy, Clone)]
struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    fn size(&self) -> f32 {
        return self.max - self.min;
    }

    fn contains(&self, x: f32) -> bool {
        return self.min <= x && x <= self.max;
    }

    fn surrounds(&self, x: f32) -> bool {
        return self.min < x && x < self.max;
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut tmax = ray_t.max;

        for obj in &self.objects {
            tmax = if let Some(temp_closest) = &closest_hit {
                temp_closest.t
            } else {
                tmax
            };
            let hit_or_none = obj.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: tmax,
                },
            );
            let Some(hit) = hit_or_none else {
                continue;
            };
            closest_hit = Some(hit);
        }
        return closest_hit;
    }
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = glm::max2_scalar(1, (image_width as f32 / aspect_ratio) as i32);

    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    // Camera

    let focal_length: f32 = 1.0;
    let camera_center: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: glm::Vec3 = glm::vec3(viewport_width, 0.0, 0.0);
    let viewport_v: glm::Vec3 = glm::vec3(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: glm::Vec3 = viewport_u / image_width as f32;
    let pixel_delta_v: glm::Vec3 = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: glm::Vec3 =
        camera_center - glm::vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: glm::Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("Hello, world!");

    let mut data: Vec<u8> = vec![0; (image_height * image_width * 4) as usize];

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

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_center: glm::Vec3 =
                pixel00_loc + i as f32 * pixel_delta_v + j as f32 * pixel_delta_u;
            let ray = Ray {
                origin: camera_center,
                direction: pixel_center - camera_center,
            };

            let _ = ray.at(0.2);

            let pixel_color = ray_color(&world, &ray);
            data[((i * image_width + j) * 4) as usize] = (pixel_color.x * 255.999) as u8;
            data[((i * image_width + j) * 4 + 1) as usize] = (pixel_color.y * 255.999) as u8;
            data[((i * image_width + j) * 4 + 2) as usize] = (pixel_color.z * 255.999) as u8;
            data[((i * image_width + j) * 4 + 3) as usize] = 255;
        }
    }
    output_file(data, image_width as u32, image_height as u32);
}
