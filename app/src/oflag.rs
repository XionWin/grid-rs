pub const READ_ONLY: libc::c_int = 0x0000;
pub const WRITE_ONLY: libc::c_int = 0x0001;
pub const READ_WRITE: libc::c_int = 0x0002;
pub const NON_BLOCK: libc::c_int = 0x0800;
pub const CLOSE_ON_EXEC: libc::c_int = 0x0080000;