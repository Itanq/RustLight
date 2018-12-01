
extern crate rand;
extern crate math;
extern crate image;

fn main() {
    draw_light(512, 512);
}

fn draw_light(width: u32, height: u32) {
    let mut img_buf = image::RgbImage::new(width, height);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let a = ((sample(x as f32 / width as f32, y as f32 / height as f32) * 255.0).min(255.0)) as u8;
        *pixel = image::Rgb([a,a,a]);
    }
    img_buf.save("light1.png").unwrap();
}

fn sample(x: f32, y: f32) -> f32 {
    let mut sum = 0.0;
    for _ in 0..64 {
        let rand: f32 =  rand::random::<f32>();
        let a: f32 = std::f32::consts::PI * 2.0 * rand;
        sum = sum + trace(x, y, a.cos(), a.sin());
    }
    sum / 64.0
}

fn trace(x: f32, y: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0;
    for _ in 0..10 {
        if t >= 2.0 {
            break;
        }
        let sd = circle_sdf(x + dx * t, y + dy * t, 0.5f32 ,0.5f32, 0.1f32);
        if sd < std::f32::EPSILON {
            return 2.0f32;
        }
        t = t + sd;
    }
    0.0
}

fn circle_sdf(x: f32, y: f32, cx: f32, cy: f32, r: f32) -> f32 {
    let ux = x - cx;
    let uy = y - cy;
    (ux * ux + uy * uy).sqrt() - r
}
