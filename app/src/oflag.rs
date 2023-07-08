#[repr(C)]
pub enum OFlag {
    READ_ONLY = 0x0000,
    WRITE_ONLY = 0x0001,
    READ_WRITE = 0x0002,
    NON_BLOCK = 0x0800,
    CLOSE_ON_EXEC = 0x0080000,
}