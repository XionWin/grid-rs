use std::time::SystemTime;

extern crate libc;

extern crate drm;

#[allow(dead_code)]
mod oflag;
mod utility;


fn main() {
    println!("====================[grid-rs]====================");
    println!("datetime: {}", utility::pretty_print_system_time(SystemTime::now()));

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