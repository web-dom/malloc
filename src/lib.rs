use std::os::raw::c_void;

#[no_mangle]
pub fn malloc(size: i32) -> *mut c_void {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr as *mut c_void;
}
