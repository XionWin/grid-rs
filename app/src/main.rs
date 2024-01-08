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

    let path = utility::get_avaliable_video_card_path().unwrap();
    println!("first_card_path: {:#?}", path);

    let fd = utility::get_fd(&path);
    println!("fd: {:#?}", fd);
    let drm = drm_rs::core::Drm::new(fd, |conn| {
        conn.get_connection_status() == drm_rs::ConnectionStatus::Connected
    });
    println!("drm handle: {:#?}", drm.handle);
    let _mode = drm.get_mode();

    let gbm = gbm_rs::Gbm::new(
        drm,
        gbm_rs::def::SurfaceFormat::ARGB8888,
        vec![gbm_rs::def::FormatModifier::DRM_FORMAT_MOD_LINEAR],
    );
    println!("gbm: {:#?}", gbm);
}
