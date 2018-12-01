
use light;

fn draw_light(path: &str, width: u32, height: u32) {
    let mut img_buf = image::RgbImage::new(width, height);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let a = ((light::sample(x as f32 / width as f32, y as f32 / height as f32) * 255.0).min(255.0)) as u8;
        *pixel = image::Rgb([a,a,a]);
    }
    img_buf.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //super::draw_light("light1.png", 512, 512);
        super::draw_light("light2.png", 512, 512)
    }
}
