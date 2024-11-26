pub mod hittable;

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
    pub origin: glm::TVec3<f64>,
    pub direction: glm::TVec3<f64>,
}

impl Ray {
    pub fn at(&self, t: f64) -> glm::TVec3<f64> {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub point: glm::TVec3<f64>,
    pub normal: glm::TVec3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn new(point: glm::TVec3<f64>, t: f64, outward_normal: glm::TVec3<f64>, r: &Ray) -> Self {
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

pub trait Random {
    fn make_random() -> Self;
    fn random_on_hemisphere(normal: &glm::TVec3<f64>) -> Self;
}

impl Random for glm::TVec3<f64> {
    fn make_random() -> Self {
        loop {
            let p = glm::vec3(
                rand::random::<f64>()-0.5,
                rand::random::<f64>()-0.5,
                rand::random::<f64>()-0.5,
            );
            let lensq: f64 = p.norm_squared();
            if f64::EPSILON < lensq && lensq <= 1.0 {
                return p / f64::sqrt(lensq);
            }
        }
    }
    fn random_on_hemisphere(normal: &glm::TVec3<f64>) -> Self {
        let on_unit_sphere = Self::make_random();
        if on_unit_sphere.dot(&normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }
}
