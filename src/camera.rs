use crate::geometry::hittable::{Hittable, HittableObject};
use crate::geometry::{HitRecord, Interval, Random, Ray};
use core::f64::INFINITY;

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub image_height: i32,      // Rendered image height
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: glm::TVec3<f64>,  // Camera center
    pixel00_loc: glm::TVec3<f64>, // Location of pixel 0, 0
    pixel_delta_u: glm::TVec3<f64>, // Offset to pixel to the right
    pixel_delta_v: glm::TVec3<f64>, // Offset to pixel below
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Camera {
        let image_height = glm::max2_scalar(1, (image_width as f64 / aspect_ratio) as i32);

        let viewport_height: f64 = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Camera

        let focal_length: f64 = 1.0;
        let camera_center: glm::TVec3<f64> = glm::vec3(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: glm::TVec3<f64> = glm::vec3(viewport_width, 0.0, 0.0);
        let viewport_v: glm::TVec3<f64> = glm::vec3(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: glm::TVec3<f64> = viewport_u / image_width as f64;
        let pixel_delta_v: glm::TVec3<f64> = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: glm::TVec3<f64> =
            camera_center - glm::vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: glm::TVec3<f64> =
            viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: (image_width as f64 / aspect_ratio) as i32,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        Ray {
            origin: self.center,
            direction: pixel_sample - self.center,
        }
    }

    fn sample_square(&self) -> glm::TVec3<f64> {
        return glm::vec3(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        );
    }

    fn ray_color(&self, obj: &HittableObject, r: &Ray, depth: i32) -> glm::TVec3<f64> {
        if depth <= 0 {
            return glm::vec3(0.0, 0.0, 0.0);
        }
        let unit_direction: glm::TVec3<f64> = r.direction.normalize();
        let closest_hit: Option<HitRecord> = obj.hit(
            r,
            Interval {
                min: 0.001,
                max: INFINITY,
            },
        );

        if let Some(closest_final) = &closest_hit {
            let direction = closest_final.normal + glm::TVec3::<f64>::random_unit_vector();
            return 0.5
                * self.ray_color(
                    &obj,
                    &Ray {
                        origin: closest_final.point,
                        direction: direction,
                    },
                    depth - 1,
                );
        }
        let a = 0.5 * unit_direction.y + 0.5;
        glm::TVec3::<f64>::new(0.5, 0.7, 1.0) * a
            + (1.0 - a) * glm::TVec3::<f64>::new(1.0, 1.0, 1.0)
    }

    pub fn render(&self, world: &HittableObject) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; (self.image_height * self.image_width * 4) as usize];

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(j, i);
                    pixel_color += self.ray_color(&world, &ray, self.max_depth);
                }

                pixel_color = self.pixel_samples_scale * pixel_color;
                let intensity = Interval {
                    min: 0.000,
                    max: 1.0,
                };
                data[((i * self.image_width + j) * 4) as usize] =
                    (intensity.clamp(pixel_color.x) * 255.999) as u8;
                data[((i * self.image_width + j) * 4 + 1) as usize] =
                    (intensity.clamp(pixel_color.y) * 255.999) as u8;
                data[((i * self.image_width + j) * 4 + 2) as usize] =
                    (intensity.clamp(pixel_color.z) * 255.999) as u8;
                data[((i * self.image_width + j) * 4 + 3) as usize] = 255;
            }
        }
        data
    }
}
