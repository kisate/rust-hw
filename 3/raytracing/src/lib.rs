use image::RgbImage;
use std::vec;

pub mod data;
use crate::data::{Light, Sphere, Vec3f};

mod raycasting;

const WIDTH: usize = 2048;
const HEIGHT: usize = 1536;
const FOV: f32 = std::f32::consts::PI / 2.5;

pub fn render(spheres: &[Sphere], lights: &[Light]) {
    let frame_buffer: &mut Vec<Vec3f> = &mut vec![Vec3f(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x =
                (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0) * f32::tan(FOV / 2.0) * WIDTH as f32
                    / HEIGHT as f32;
            let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * f32::tan(FOV / 2.0);
            let dir = Vec3f(x, y, -1.0).normalize();
            frame_buffer[i + j * WIDTH] = raycasting::cast_ray(&Vec3f(0.0, 0.0, 0.0), &dir, spheres, lights, 0);
        }
    }

    let img = RgbImage::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        image::Rgb(frame_buffer[(x as usize + (y as usize) * WIDTH)].to_bytes())
    });

    img.save("test.png").unwrap();
}
