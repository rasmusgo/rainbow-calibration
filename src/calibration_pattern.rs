use std::f32::consts::TAU;

use image::{ImageBuffer, Rgb};

pub fn generate_calibration_pattern(
    img_width: u32,
    img_height: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let scale_x = TAU / 100.0 as f32;
    let scale_y = TAU / 100.0 as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(img_width, img_height);

    // Iterate over the coordinates and pixels of the image
    let a_dir = ((30.0 + 120.0 * 1.0) / 360.0 * TAU).sin_cos();
    let b_dir = ((30.0 + 120.0 * 2.0) / 360.0 * TAU).sin_cos();
    let c_dir = ((30.0 + 120.0 * 3.0) / 360.0 * TAU).sin_cos();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let scaled_x = x as f32 * scale_x;
        let scaled_y = y as f32 * scale_y;
        let a = (a_dir.0 * scaled_x) + (a_dir.1 * scaled_y);
        let b = (b_dir.0 * scaled_x) + (b_dir.1 * scaled_y);
        let c = (c_dir.0 * scaled_x) + (c_dir.1 * scaled_y);
        let r = ((a.sin() + 1.0) * (255.0 / 2.0)) as u8;
        let g = ((b.sin() + 1.0) * (255.0 / 2.0)) as u8;
        let b = ((c.sin() + 1.0) * (255.0 / 2.0)) as u8;
        *pixel = Rgb([r, g, b]);
    }

    // Save the image, the format is deduced from the path
    // imgbuf.save("calibration_pattern.png").unwrap();
    imgbuf
}
