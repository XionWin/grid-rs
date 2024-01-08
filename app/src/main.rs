use std::time::SystemTime;

use colored::Colorize;
use rand::Rng;

extern crate libc;
extern crate drm;

#[allow(dead_code)]
mod oflag;
mod utility;

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", "====================[grid-rs]====================".truecolor(rng.gen(), rng.gen(), rng.gen()));
    println!("datetime: {}", utility::pretty_print_system_time(SystemTime::now()).green());

    let path = utility::get_avaliable_video_card_path().unwrap();
    println!("first_card_path: {:#?}", path);

    let fd = utility::get_fd(&path);
    println!("fd: {:#?}", fd);
    let drm = drm::core::Drm::new(
        fd,
        |conn| conn.get_connection_status() == drm::ConnectionStatus::Connected
    );

    println!("drm handle: {:#?}", drm.handle);
    let _mode = drm.get_mode();
}