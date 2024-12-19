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
    DielectricObject(Dielectric),
}

impl<'a> Material<'a> for MaterialObject {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'a>, attenuation: &mut Vec3) -> Option<Ray> {
        match self {
            Self::MetalObject(metal) => metal.scatter(r_in, rec, attenuation),
            Self::LambertianObject(lambertian) => lambertian.scatter(r_in, rec, attenuation),
            Self::DielectricObject(dielectric) => dielectric.scatter(r_in, rec, attenuation),
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord<'a>, attenuation: &mut Vec3) -> Option<Ray> {
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

pub struct Dielectric {
    pub refraction_index: f64,
}
impl Dielectric {
    fn reflectance(&self, cosine: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl<'a> Material<'a> for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'a>, attenuation: &mut Vec3) -> Option<Ray> {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.normalize();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();


        let cannot_refract = ri * sin_theta > 1.0;


        let mut direction;
        if  cannot_refract || self.reflectance(cos_theta) > rand::random::<f64>(){
            direction = r_in.direction.reflect(&rec.normal);
        } else {
            direction = unit_direction.refract(&rec.normal, ri);
        }
        return Some(Ray {
            origin: rec.point,
            direction: direction,
        });

    }
}
