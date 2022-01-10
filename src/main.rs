use std::ops::{Deref, DerefMut};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};
use nalgebra_glm::pi;


struct Ray {
    origin:    Point3<f64>,
    direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&mut self) -> &mut Point3<f64> {
        &mut self.origin
    }
    pub fn direction(&mut self) -> &mut Vector3<f64> {
        &mut self.direction
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }
}



fn ray_color(r: &Ray) -> Vector3<f64> {
    let unit_direction: Vector3<f64> = r.direction / (r.direction.len() as f64);
    let t: f64 = 0.5*(unit_direction.y + 1.0);
    // let u = 1.0 - t;
    ((1.0 - t)*Vector3::new(1.0, 1.0, 1.0)) + (t*Vector3::new(0.5, 0.7, 1.0))
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vector3<f64> = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical: Vector3<f64> = Vector3::new(0.0, viewport_height, 0.0);

    let horz_half: Vector3<f64> = horizontal / 2.0;
    let vert_half: Vector3<f64> = vertical/2.0;
    let end_vec: Vector3<f64> = Vector3::new(0.0, 0.0, focal_length);
    let lower_left_corner = origin - horz_half - vert_half - end_vec;

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);

    let mut j = image_height-1;
    while j >= 0 {

        let mut i = 0;
        while i < image_width {

            let u: f64 = (i as f64 / (image_width - 1) as f64);
            let v: f64 = (j as f64 / (image_height - 1) as f64);

            let r = Ray{origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);

            // let r = i as f64 / (image_width-1) as f64;
            // let g = j as f64 / (image_height-1) as f64;
            // let b = 0.25;
            // let pixel_color: Vector3<f64> = Vector3::new(r, g, b);
            // let v = Vector3::new(r, g, b);
            // write_color(&pixel_color);

            i = i+1;
        }

        j = j-1;
        bar.inc(1);
    }
}


fn write_color(pixel_color: &Vector3<f64>) {
    let x = (pixel_color.x * 255.999) as i64;
    let y = (pixel_color.y * 255.999) as i64;
    let z = (pixel_color.z * 255.999) as i64;

    println!("{} {} {}", x, y, z);
}
