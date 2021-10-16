use std::mem::swap;

use crate::data::{Light, Material, Sphere, Vec3f};

const EPS: f32 = 1e-3;
const PLANE_COLOR_1: Vec3f = Vec3f(0.3, 0.3, 0.3);
const PLANE_COLOR_2: Vec3f = Vec3f(0.3, 0.2, 0.1);
const BG_COLOR: Vec3f = Vec3f(0.2, 0.7, 0.8);
const MAXIMAL_DIST: f32 = 1000.0;
const MAX_CASTING_DEPTH: usize = 4;

fn reflect(ray: &Vec3f, normal: &Vec3f) -> Vec3f {
    *ray - *normal * 2.0 * (*ray * *normal)
}

fn refract(ray: &Vec3f, mut normal: Vec3f, refractive_index: f32) -> Vec3f {
    let mut cosi = (-1.0) * f32::max(-1.0, f32::min(1.0, *ray * normal));
    let mut etai: f32 = 1.0;
    let mut etat = refractive_index;

    if cosi < 0.0 {
        cosi *= -1.0;
        swap(&mut etai, &mut etat);
        normal = normal * (-1.0);
    };

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3f(1.0, 0.0, 0.0)
    } else {
        *ray * eta + normal * (eta * cosi - f32::sqrt(k))
    }
}

fn intersect_spheres(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    hit: &mut Vec3f,
    normal: &mut Vec3f,
    material: &mut Material,
) -> f32 {
    let mut spheres_dist = f32::MAX;
    for sphere in spheres {
        let mut dist_i: f32 = 0.0;
        if sphere.ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
            spheres_dist = dist_i;
            *hit = *orig + *dir * dist_i;
            *normal = (*hit - sphere.center).normalize();
            *material = sphere.material;
        }
    }
    return spheres_dist;
}

fn intersect_plane(
    orig: &Vec3f,
    dir: &Vec3f,
    hit: &mut Vec3f,
    normal: &mut Vec3f,
    material: &mut Material,
    spheres_dist: f32,
) -> f32 {
    let mut checkerboard_dist = f32::MAX;
    if dir.1.abs() > EPS {
        let d = -(orig.1 + 4.0) / dir.1;
        let pt = *orig + *dir * d;

        if d > 0.0 && pt.0.abs() < 10.0 && pt.2 < -10.0 && pt.2 > -30.0 && d < spheres_dist {
            checkerboard_dist = d;
            *hit = pt;
            *normal = Vec3f(0.0, 1.0, 0.0);
            material.diffuse_color =
                if ((0.5 * hit.0 + 1000.0) as i32 + (0.5 * hit.2) as i32) & 1 > 0 {
                    PLANE_COLOR_1
                } else {
                    PLANE_COLOR_2
                };
        }
    }
    return checkerboard_dist;
}

/// Intersect all objects with ray
/// # Arguments
/// * `orig` - ray origin
/// * `dir` - ray direction
/// * `spheres` - array of spheres
/// * `hit` - point of hit
/// * `normal` - normal in point of hit
/// * `material` - material in point of hit
fn scene_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    hit: &mut Vec3f,
    normal: &mut Vec3f,
    material: &mut Material,
) -> bool {
    let spheres_dist = intersect_spheres(orig, dir, spheres, hit, normal, material);
    let checkerboard_dist = intersect_plane(orig, dir, hit, normal, material, spheres_dist);
    f32::min(spheres_dist, checkerboard_dist) < MAXIMAL_DIST
}


/// Cast ray on the spheres recursively
/// # Arguments
/// * `orig` - ray origin
/// * `dir` - ray direction
/// * `spheres` - array of spheres
/// * `lights` - array of lights
/// * `depth` - recursion depth (should be 0 initially)
pub fn cast_ray(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    lights: &[Light],
    depth: usize,
) -> Vec3f {
    let mut point = Vec3f::default();
    let mut normal = Vec3f::default();
    let mut material = Material::default();

    if depth > MAX_CASTING_DEPTH || !scene_intersect(orig, dir, spheres, &mut point, &mut normal, &mut material) {
        return BG_COLOR;
    }

    let reflect_dir = reflect(dir, &normal);
    let refract_dir = refract(dir, normal, material.refractive_index).normalize();

    let reflect_orig = if reflect_dir * normal < 0.0 {
        point - normal * EPS
    } else {
        point + normal * EPS
    };
    let refract_orig = if refract_dir * normal < 0.0 {
        point - normal * EPS
    } else {
        point + normal * EPS
    };

    let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, depth + 1);
    let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, depth + 1);

    let mut diffuse_light_intensity: f32 = 0.0;
    let mut specular_light_intensity: f32 = 0.0;
    for light in lights {
        let light_dir = (light.position - point).normalize();
        let light_distance = (light.position - point).norm();

        let shadow_orig = if light_dir * normal < 0.0 {
            point - normal * EPS
        } else {
            point + normal * EPS
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

        diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir * normal);
        specular_light_intensity += f32::max(0.0, reflect(&light_dir, &normal) * *dir)
            .powf(material.specular_component)
            * light.intensity
    }

    material.diffuse_color * diffuse_light_intensity * material.albedo[0]
        + Vec3f(1., 1., 1.) * specular_light_intensity * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[3]
}
