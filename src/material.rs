use crate::{
    geometry::{HitRecord, Ray, Vec3Utils},
    Vec3,
};

// TODO: this seems too circular, pass the hit record containing the material to the material??
pub trait Material<'a> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'a>, attenuation: &mut Vec3) -> Option<Ray>;
}

pub enum MaterialObject {
    MetalObject(Metal),
    LambertianObject(Lambertian),
}

impl<'a> Material<'a> for MaterialObject {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'a>, attenuation: &mut Vec3) -> Option<Ray> {
        match self {
            Self::MetalObject(metal) => metal.scatter(r_in, rec, attenuation),
            Self::LambertianObject(lambertian) => lambertian.scatter(r_in, rec, attenuation),
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl<'a> Material<'a> for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'a>, attenuation: &mut Vec3) -> Option<Ray> {
        let reflected = r_in.direction.reflect(&rec.normal);
        let fuzzed = reflected.normalize() + self.fuzz * Vec3::random_unit_vector();
        *attenuation = self.albedo;
        if fuzzed.dot(&rec.normal) > 0.0 {
            let scattered = Ray {
                direction: fuzzed,
                origin: rec.point,
            };
            return Some(scattered);
        }
        None
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl<'a> Material<'a> for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord<'a>,
        attenuation: &mut Vec3, //TODO: Some way to do this without an input/output parameter?
    ) -> Option<Ray> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *attenuation = self.albedo;
        Some(Ray {
            origin: rec.point,
            direction: scatter_direction,
        })
    }
}
