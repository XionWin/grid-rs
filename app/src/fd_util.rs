use crate::oflag;

pub fn get_avaliable_video_card_path() -> Option<String> {
    let validated_card_pathes = std::fs::read_dir("/dev/dri").unwrap()
    .map(|x| {
        let os_path = x.as_ref().unwrap().path();
        let card_path = String::from(os_path.to_str().unwrap());

        println!("card_path:{:#?}", card_path);
        let fd = get_fd(&card_path);
        println!("fd:{:#?}", fd);

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

    if validated_card_pathes.len() > 0 {
        Some(String::from(validated_card_pathes.first().unwrap()))
    }
    else {
        Option::None
    }
}

pub fn get_fd(device_path: &str) -> libc::c_int {
    let path = device_path.bytes().collect::<Vec<libc::c_char>>();
    unsafe {
        libc::open(path.as_ptr(), oflag::OFlag::ReadWrite as _)
    }
}