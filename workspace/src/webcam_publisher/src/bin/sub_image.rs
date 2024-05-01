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
    let node = ctx.create_node("image_subscriber", None, Default::default())?;
    // subscriber
    let subscriber = node.create_subscriber::<msg::Image>("webcam_image", None)?;
    let logger = Logger::new("my_listener");

    let mut selector = ctx.create_selector()?;
    selector.add_subscriber(
        subscriber,
        Box::new(move |msg| {
            let image = &msg.data;
            pr_info!(logger, "receive: {:?}", image)
        })
    );

    loop {
        selector.wait()?;
    }
}

