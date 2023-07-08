extern crate libc;

extern crate drm;

#[allow(dead_code)]
mod oflag;

fn main() {
    println!("Hello, world!");
    let fd = get_fd("/dev/dri/card0");
    println!("fd:{:#?}", fd);

    let drm = drm::core::Drm::new(
        fd,
        |conn| conn.get_connection_status() == drm::ConnectionStatus::Connected
    );
    println!("[DRM] {:#?}", drm);

    let mode = drm.get_mode();
    println!("[DRM MODE] {:#?}", mode);

}

fn get_fd(device_path: &str) -> libc::c_int {
    let path = device_path.bytes().collect::<Vec<libc::c_char>>();
    unsafe {
        libc::open(path.as_ptr(), oflag::OFlag::READ_WRITE as _)
    }
}
