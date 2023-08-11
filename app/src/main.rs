extern crate libc;

extern crate drm;

#[allow(dead_code)]
mod oflag;

fn main() {
    println!("Hello, world!");
    let validated_card_pathes = std::fs::read_dir("/dev/dri").unwrap()
    .map(|x| {
        let os_path = x.as_ref().unwrap().path();
        let card_path = String::from(os_path.to_str().unwrap());

        let fd = get_fd(&card_path);

        let is_name_contains = x.as_ref().unwrap().file_name().to_str().unwrap().contains("card");
        let is_validated = drm::core::is_validated_handle(fd);

        match is_name_contains && is_validated  {
            true => Some(card_path),
            false => Option::None,
        }

    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<String>>();


    let path = validated_card_pathes.first().unwrap();
    println!("first_card_path:{:#?}", path);

    let fd = get_fd(&path);
    println!("fd:{:#?}", fd);
    let drm = drm::core::Drm::new(
        fd,
        |conn| conn.get_connection_status() == drm::ConnectionStatus::Connected
    );

    let mode = drm.get_mode();
    println!("[DRM HANDLE] {:#?}", drm.handle);
    println!("[DRM MODE] {:#?}", mode);

}

fn get_fd(device_path: &str) -> libc::c_int {
    let path = device_path.bytes().collect::<Vec<libc::c_char>>();
    unsafe {
        libc::open(path.as_ptr(), oflag::OFlag::ReadWrite as _)
    }
}
