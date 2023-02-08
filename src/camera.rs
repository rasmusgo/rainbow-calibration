use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

pub fn capture_camera_image() {
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
