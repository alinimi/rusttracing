use crate::{
    geometry::{HitRecord, Interval, Ray},
    material::MaterialObject,
    Vec3,
};

pub trait Hittable<'a> {
    fn hit(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>>;
}

pub enum HittableObject {
    SphereObject(Sphere),
    HittableListObject(HittableList),
}
impl<'a> Hittable<'a> for HittableObject {
    fn hit(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
        match self {
            Self::SphereObject(sphere) => sphere.hit(r, ray_t),
            Self::HittableListObject(hittable_list) => hittable_list.hit(r, ray_t),
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    material: MaterialObject,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: MaterialObject) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}
impl<'a> Hittable<'a> for Sphere {
    fn hit(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
        let oc: Vec3 = self.center - r.origin;
        let a = glm::length2(&r.direction);
        let h = glm::dot(&r.direction, &oc);
        let c = glm::length2(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        // let mut hitv: HitRecord = HitRecord{};
        let root0 = (h - f64::sqrt(discriminant)) / a;
        let root1 = (h + f64::sqrt(discriminant)) / a;

        let t: f64;

        if ray_t.surrounds(root0) {
            t = root0;
        } else if ray_t.surrounds(root1) {
            t = root1;
        } else {
            return None;
        }

        let point = r.at(t);
        let hit = HitRecord::new(
            point,
            t,
            (point - self.center).normalize(),
            r,
            &self.material,
        );
        Some(hit)
    }
}

pub struct HittableList {
    pub objects: Vec<HittableObject>,
}
impl HittableList {
    pub fn add(&mut self, obj: HittableObject) {
        self.objects.push(obj);
    }
}
impl<'a> Hittable<'a> for HittableList {
    fn hit(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
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
