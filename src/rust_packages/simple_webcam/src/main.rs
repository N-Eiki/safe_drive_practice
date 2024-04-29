
extern crate image;
extern crate imageproc;
extern crate rscam;

// show images
extern crate show_image; //::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() {
    let device = "/dev/video0";
    let mut camera = rscam::new(device).unwrap();
    // setting camera params
    let height = 1920;
    let width = 1080;
    let format = b"RGB3";
    let fps = 30;
    // create window for images
    let window = show_image::create_window("image", Default::default()).unwrap();

    // capture camera
    camera
        .start(&rscam::Config {
            interval: (1, fps),
            resolution: (height, width),
            format: format,
            ..Default::default()
        })
        .unwrap();

        // show image loop
        loop {
            let frame = camera.capture().unwrap();
            let image = show_image::ImageView::new(show_image::ImageInfo::rgb8(height, width), &frame);
            window.set_image("image-001", &image);
        }
}

