pub mod hittable;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn size(&self) -> f32 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f32) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f32) -> bool {
        return self.min < x && x < self.max;
    }
}

pub struct Ray {
    pub origin: glm::Vec3,
    pub direction: glm::Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> glm::Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub point: glm::Vec3,
    pub normal: glm::Vec3,
    pub t: f32,
    pub front_face: bool,
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
