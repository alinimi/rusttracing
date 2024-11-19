use crate::geometry::hittable::{Hittable, HittableObject};
use crate::geometry::{HitRecord, Interval, Ray};

use core::f32::INFINITY;

pub struct Camera {
    pub aspect_ratio: f32,    // Ratio of image width over height
    pub image_width: i32,     // Rendered image width in pixel count
    pub image_height: i32,    // Rendered image height
    center: glm::Vec3,        // Camera center
    pixel00_loc: glm::Vec3,   // Location of pixel 0, 0
    pixel_delta_u: glm::Vec3, // Offset to pixel to the right
    pixel_delta_v: glm::Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32) -> Camera {
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

        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: (image_width as f32 / aspect_ratio) as i32,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableObject) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; (self.image_height * self.image_width * 4) as usize];

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let pixel_center: glm::Vec3 = self.pixel00_loc
                    + i as f32 * self.pixel_delta_v
                    + j as f32 * self.pixel_delta_u;
                let ray = Ray {
                    origin: self.center,
                    direction: pixel_center - self.center,
                };

                let _ = ray.at(0.2);

                let pixel_color = self.ray_color(&world, &ray);
                data[((i * self.image_width + j) * 4) as usize] = (pixel_color.x * 255.999) as u8;
                data[((i * self.image_width + j) * 4 + 1) as usize] =
                    (pixel_color.y * 255.999) as u8;
                data[((i * self.image_width + j) * 4 + 2) as usize] =
                    (pixel_color.z * 255.999) as u8;
                data[((i * self.image_width + j) * 4 + 3) as usize] = 255;
            }
        }
        data
    }

    fn ray_color(&self, obj: &HittableObject, r: &Ray) -> glm::Vec3 {
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
}
