use std::f32::consts::TAU;

use image;
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

fn generate_calibration_pattern() {
    let img_width = 1280;
    let img_height = 1280;

    let scale_x = TAU * 30.0 / img_width as f32;
    let scale_y = TAU * 30.0 / img_height as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(img_width, img_height);

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
        *pixel = image::Rgb([r, g, b]);
    }

    // Save the image, the format is deduced from the path
    imgbuf.save("calibration_pattern.png").unwrap();
}

fn main() {
    // first camera in system
    let index = CameraIndex::Index(0);
    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    // make the camera
    let mut camera = Camera::new(index, requested).unwrap();

    // get a frame
    let frame = camera.frame().unwrap();
    println!("Captured Single Frame of {}", frame.buffer().len());
    // decode into an ImageBuffer
    let decoded = frame.decode_image::<RgbFormat>().unwrap();
    println!("Decoded Frame of {}", decoded.len());

    decoded.save("camera_image.png").unwrap();
}
