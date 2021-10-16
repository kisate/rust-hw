use std::ops;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    pub fn to_bytes(self) -> [u8; 3] {
        [
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8,
        ]
    }

    pub fn norm(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    pub fn normalize(self) -> Vec3f {
        let norm = self.norm();
        Vec3f(self.0 / norm, self.1 / norm, self.2 / norm)
    }
}

impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Self::Output {
        Vec3f(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Self::Output {
        Vec3f(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<Vec3f> for Vec3f {
    type Output = f32;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3f(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse_color: Vec3f,
    pub albedo: [f32; 4],
    pub specular_component: f32,
    pub refractive_index: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse_color: Default::default(),
            albedo: [1.0, 0.0, 0.0, 0.0],
            specular_component: Default::default(),
            refractive_index: 1.0,
        }
    }
}

#[derive(Default)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    /// Checks if ray intersects the sphere
    /// # Arguments
    /// * `orig` - ray origin
    /// * `dir` - ray direction
    /// * `t0` - place to write distance from origin
    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f, t0: &mut f32) -> bool {
        let l = self.center - *orig;
        let tca = l * *dir;
        let d2 = l * l - tca * tca;
        if d2 > self.radius * self.radius {
            return false;
        }
        let thc = f32::sqrt(self.radius * self.radius - d2);
        *t0 = tca - thc;
        let t1 = tca + thc;
        if *t0 < 0.0 {
            *t0 = t1;
        }
        if *t0 < 0.0 {
            return false;
        }
        true
    }
}
