use safe_drive::{
    context::Context, error::DynError, logger::Logger, pr_info
};
use safe_drive;
use safe_drive::msg::RosString;
use safe_drive::msg::U8Seq;

use std::time::Duration;

extern crate image;
extern crate imageproc;
extern crate rscam;
use sensor_msgs::msg;

// show images
extern crate show_image; //::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() -> Result<(), DynError> {
    // コンテキストの作成
    let ctx = Context::new()?;
    // ノードを作成
    let node = ctx.create_node("image_publisher", None, Default::default())?;
    // publisherの作成
    let publisher = node.create_publisher::<msg::Image>("webcam_image", None)?;

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
            // let image = show_image::ImageView::new(show_image::ImageInfo::rgb8(height, width), &frame);
            // let _ = window.set_image("image-001", &image);

            let mut image_msg = msg::Image::new().expect("Failed to create Image message instance");
            let header_ros_string = RosString::<0>::new("camera_frame").unwrap();
            let encoding_ros_string = RosString::<0>::new("rgb8").unwrap();
            let mut img_data = U8Seq::new(frame.len()).unwrap();
            img_data.as_slice_mut().copy_from_slice(&frame.to_vec());

            image_msg.header.frame_id = header_ros_string;
            image_msg.height = height;
            image_msg.width = width;
            image_msg.encoding = encoding_ros_string;
            image_msg.data = img_data; // キャプチャしたフレームをバイト列に変換して代入
            let _ = publisher.send(&image_msg);
        }
}

