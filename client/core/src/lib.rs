use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn send_message(text: *const c_char) -> *mut c_char {
    return CString::new("Hello world").unwrap().into_raw();
}
