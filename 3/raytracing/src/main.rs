use raytracing::{render, Light, Material, Sphere, Vec3f};
fn main() {
    let ivory = Material {
        diffuse_color: Vec3f(0.4, 0.4, 0.3),
        albedo: [0.6, 0.3, 0.1, 0.0],
        specular_component: 50.0,
        refractive_index: 1.0,
    };
    let red_rubber = Material {
        diffuse_color: Vec3f(0.3, 0.1, 0.1),
        albedo: [0.9, 0.1, 0.0, 0.0],
        specular_component: 10.0,
        refractive_index: 1.0,
    };
    let mirror = Material {
        diffuse_color: Vec3f(1.0, 1.0, 1.0),
        albedo: [0.0, 10.0, 0.8, 0.0],
        specular_component: 1425.0,
        refractive_index: 1.0,
    };
    let glass = Material {
        diffuse_color: Vec3f(0.6, 0.7, 0.8),
        albedo: [0.0, 0.5, 0.1, 0.8],
        specular_component: 125.0,
        refractive_index: 1.0,
    };

    let spheres = vec![
        Sphere {
            center: Vec3f(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory,
        },
        Sphere {
            center: Vec3f(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: glass,
        },
        Sphere {
            center: Vec3f(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber,
        },
        Sphere {
            center: Vec3f(7.0, 5.0, -18.0),
            radius: 4.0,
            material: mirror,
        },
    ];

    let lights = vec![
        Light {
            position: Vec3f(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        Light {
            position: Vec3f(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        Light {
            position: Vec3f(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];

    render(&spheres, &lights);
}
