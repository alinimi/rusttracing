pub mod hittable;
use crate::material::MaterialObject;
use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        };
        if x > self.max {
            return self.max;
        };
        return x;
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a MaterialObject,
}

impl<'a> HitRecord<'a> {
    fn new(
        point: Vec3,
        t: f64,
        outward_normal: Vec3,
        r: &Ray,
        material: &'a MaterialObject,
    ) -> Self {
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
            material: material,
        }
    }
}

pub trait Vec3Utils {
    fn random_unit_vector() -> Self;
    fn random_on_hemisphere(normal: &Vec3) -> Self;
    fn near_zero(&self) -> bool;
    fn reflect(&self, n: &Vec3) -> Self;
}

impl Vec3Utils for Vec3 {
    fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::new(
                rand::random::<f64>() - 0.5,
                rand::random::<f64>() - 0.5,
                rand::random::<f64>() - 0.5,
            );
            let lensq: f64 = p.norm_squared();
            if f64::EPSILON < lensq && lensq <= 1.0 {
                return p / f64::sqrt(lensq);
            }
        }
    }
    fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }
    fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
    }
    fn reflect(&self, n: &Vec3) -> Self {
        self - 2.0 * self.dot(n) * n
    }
}
