use color_rs::Colorize;
use rand::Rng;
use std::time::SystemTime;

extern crate drm_rs;
extern crate gbm_rs;
extern crate libc;

#[allow(dead_code)]
mod oflag;
mod utility;

fn main() {
    let mut rng = rand::thread_rng();
    println!(
        "{}",
        "====================[grid-rs]====================".truecolor(
            rng.gen(),
            rng.gen(),
            rng.gen()
        )
    );
    println!(
        "datetime: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );

    let video_card_info = utility::get_avaliable_video_card().unwrap();
    println!("video_card_info: {:#?}", video_card_info);

    let fd = video_card_info.fd;
    let drm = drm_rs::core::Drm::new(fd, |conn| {
        conn.get_connection_status() == drm_rs::ConnectionStatus::Connected
    });
    let _mode = drm.get_mode();

    let gbm = gbm_rs::Gbm::new(
        drm,
        gbm_rs::def::SurfaceFormat::ARGB8888,
        vec![gbm_rs::def::FormatModifier::DRM_FORMAT_MOD_LINEAR],
    );
    // println!("gbm: {:#?}", gbm);

    let supported_surface_format = gbm_rs::def::SurfaceFormat::iter().into_iter().filter(
        |format| {
            gbm.get_surface().get_device().is_format_supported(*format, gbm_rs::def::SurfaceFlags::Linear)
        } 
    ).collect::<Vec<gbm_rs::def::SurfaceFormat>>();

    supported_surface_format.into_iter().for_each(
        |format| println!("{:?}", format)
    );


} 
