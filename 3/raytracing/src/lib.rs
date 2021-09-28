use image::RgbImage;
use std::{fmt::Debug, mem::swap, ops, vec};

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    fn to_bytes(self) -> [u8; 3] {
        [
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8,
        ]
    }

    fn norm(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    fn normalize(self) -> Vec3f {
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
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f, t0: &mut f32) -> bool {
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

fn reflect(i: &Vec3f, n: &Vec3f) -> Vec3f {
    *i - *n * 2.0 * (*i * *n)
}

fn refract(i: &Vec3f, mut n: Vec3f, refractive_index: f32) -> Vec3f {
    let mut cosi = (-1.0) * f32::max(-1.0, f32::min(1.0, *i * n));
    let mut etai: f32 = 1.0;
    let mut etat = refractive_index;

    if cosi < 0.0 {
        cosi *= -1.0;
        swap(&mut etai, &mut etat);
        n = n * (-1.0);
    };

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3f(1.0, 0.0, 0.0)
    } else {
        *i * eta + n * (eta * cosi - f32::sqrt(k))
    }
}

fn scene_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    hit: &mut Vec3f,
    n: &mut Vec3f,
    material: &mut Material,
) -> bool {
    let mut spheres_dist = f32::MAX;
    for sphere in spheres {
        let mut dist_i: f32 = 0.0;
        if sphere.ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
            spheres_dist = dist_i;
            *hit = *orig + *dir * dist_i;
            *n = (*hit - sphere.center).normalize();
            *material = sphere.material;
        }
    }

    let mut checkerboard_dist = f32::MAX;
    if dir.1.abs() > 1e-3 {
        let d = -(orig.1 + 4.0) / dir.1;
        let pt = *orig + *dir * d;

        if d > 0.0 && pt.0.abs() < 10.0 && pt.2 < -10.0 && pt.2 > -30.0 && d < spheres_dist {
            checkerboard_dist = d;
            *hit = pt;
            *n = Vec3f(0.0, 1.0, 0.0);
            material.diffuse_color =
                if ((0.5 * hit.0 + 1000.0) as i32 + (0.5 * hit.2) as i32) & 1 > 0 {
                    Vec3f(0.3, 0.3, 0.3)
                } else {
                    Vec3f(0.3, 0.2, 0.1)
                };
        }
    }
    f32::min(spheres_dist, checkerboard_dist) < 1000.0
}

fn cast_ray(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    lights: &[Light],
    depth: usize,
) -> Vec3f {
    let mut point = Vec3f::default();
    let mut n = Vec3f::default();
    let mut material = Material::default();

    if depth > 4 || !scene_intersect(orig, dir, spheres, &mut point, &mut n, &mut material) {
        return Vec3f(0.2, 0.7, 0.8);
    }

    let reflect_dir = reflect(dir, &n);
    let refract_dir = refract(dir, n, material.refractive_index).normalize();

    let reflect_orig = if reflect_dir * n < 0.0 {
        point - n * 1e-3
    } else {
        point + n * 1e-3
    };
    let refract_orig = if refract_dir * n < 0.0 {
        point - n * 1e-3
    } else {
        point + n * 1e-3
    };

    let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, depth + 1);
    let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, depth + 1);

    let mut diffuse_light_intensity: f32 = 0.0;
    let mut specular_light_intensity: f32 = 0.0;
    for light in lights {
        let light_dir = (light.position - point).normalize();
        let light_distance = (light.position - point).norm();

        let shadow_orig = if light_dir * n < 0.0 {
            point - n * 1e-3
        } else {
            point + n * 1e-3
        };
        let mut shadow_pt = Vec3f::default();
        let mut shadow_n = Vec3f::default();
        let mut tmp_material = Material::default();

        if scene_intersect(
            &shadow_orig,
            &light_dir,
            spheres,
            &mut shadow_pt,
            &mut shadow_n,
            &mut tmp_material,
        ) && (shadow_pt - shadow_orig).norm() < light_distance
        {
            continue;
        }

        diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir * n);
        specular_light_intensity += f32::max(0.0, reflect(&light_dir, &n) * *dir)
            .powf(material.specular_component)
            * light.intensity
    }

    material.diffuse_color * diffuse_light_intensity * material.albedo[0]
        + Vec3f(1., 1., 1.) * specular_light_intensity * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[3]
}

pub fn render(spheres: &[Sphere], lights: &[Light]) {
    let width = 2048;
    let height = 1536;
    let fov = std::f32::consts::PI / 2.5;
    let frame_buffer: &mut Vec<Vec3f> = &mut vec![Vec3f(0.0, 0.0, 0.0); width * height];

    for j in 0..height {
        for i in 0..width {
            let x =
                (2.0 * (i as f32 + 0.5) / width as f32 - 1.0) * f32::tan(fov / 2.0) * width as f32
                    / height as f32;
            let y = -(2.0 * (j as f32 + 0.5) / height as f32 - 1.0) * f32::tan(fov / 2.0);
            let dir = Vec3f(x, y, -1.0).normalize();
            frame_buffer[i + j * width] = cast_ray(&Vec3f(0.0, 0.0, 0.0), &dir, spheres, lights, 0);
        }
    }

    let img = RgbImage::from_fn(width as u32, height as u32, |x, y| {
        image::Rgb(frame_buffer[(x as usize + (y as usize) * width)].to_bytes())
    });

    img.save("test.png").unwrap();
}
