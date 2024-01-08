extern crate libc;

extern crate drm;

#[allow(dead_code)]
mod oflag;

mod fd_util;

fn main() {
    println!("Hello, world!");

    let path = fd_util::get_avaliable_video_card_path().unwrap();
    println!("first_card_path:{:#?}", path);

    let fd = fd_util::get_fd(&path);
    println!("fd:{:#?}", fd);
    let drm = drm::core::Drm::new(
        fd,
        |conn| conn.get_connection_status() == drm::ConnectionStatus::Connected
    );

    let mode = drm.get_mode();
    println!("[DRM HANDLE] {:#?}", drm.handle);
    println!("[DRM MODE] {:#?}", mode); 

}

