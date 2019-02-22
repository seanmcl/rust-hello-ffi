extern crate libc;

use std::ffi::CStr;
use libc::c_char;

#[no_mangle]
pub extern "C" fn length(x: *const c_char, y: *const c_char) -> i32 {
    let r_x = unsafe { CStr::from_ptr(x).to_string_lossy().into_owned() };
    let r_y = unsafe { CStr::from_ptr(y).to_string_lossy().into_owned() };
    (r_x.chars().count() + r_y.chars().count()) as i32
}
