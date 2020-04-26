use std::{slice, str};

const ADMIT_BUFFER_SIZE: usize = std::mem::size_of::<u16>();
static mut ADMIT_BUFFER: [u8; ADMIT_BUFFER_SIZE] = [0; ADMIT_BUFFER_SIZE];

#[no_mangle]
pub fn admit_ptr() -> *const u8 {
    unsafe { ADMIT_BUFFER.as_ptr() }
}

#[no_mangle]
pub fn admit(ptr: *const u8, len: u16) -> u8 {
    if read_str(ptr, len) == "password" {
        return 1;
    }

    0
}

fn read_str(ptr: *const u8, sz: u16) -> String {
    let slice = unsafe { slice::from_raw_parts(ptr, sz as usize) };
    str::from_utf8(&slice).unwrap().to_string()
}
