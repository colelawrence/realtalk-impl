use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

mod system;
mod data;

use system::System;

#[no_mangle]
pub extern "C" fn im_system_debug(sys_ptr: *mut System) {
    assert!(!sys_ptr.is_null()); // this is not enough...
    let system = unsafe { &mut *sys_ptr };
    system::system_impl_debug(system);
}

/// Make sure to forget the system when you are done with it!
#[no_mangle]
pub extern "C" fn im_system_new(name: *const c_char) -> *mut System {
    // borrowed from https://medium.com/jim-fleming/complex-types-with-rust-s-ffi-315d14619479
    let cstr = unsafe { CStr::from_ptr(name) };
    let name_str = cstr.to_str().expect("valid utf8");
    let system = unsafe { std::mem::transmute(Box::new(System::new(name_str))) };
    system
}

#[no_mangle]
pub extern "C" fn im_system_drop(sys_ptr: *mut System) {
    // let mut system = unsafe { &mut *sys_ptr };
    let sys: Box<System> = unsafe { std::mem::transmute(sys_ptr) };
    println!("Dropping system: {:?}", sys);
}

#[no_mangle]
pub extern "C" fn im_system_tests(sys_ptr: *mut System) {
    assert!(!sys_ptr.is_null()); // this is not enough...
    system::system_impl_tests(unsafe { &mut *sys_ptr });
}

#[no_mangle]
pub extern "C" fn reverse_free(s: *mut c_char) {
    unsafe { CString::from_raw(s) };
}

#[no_mangle]
// pub extern "C" fn test(source: *const c_char, dest: *mut c_char) -> size_t {
pub extern "C" fn reverse_call(source: *const c_char) -> *mut c_char {
    let source_cstr = unsafe { CStr::from_ptr(source) };
    // println!("cstr: {:?}", source_cstr);
    let res = reverse(source_cstr.to_str().expect("valid utf8"));
    let res_cstr = CString::new(res.as_bytes()).expect("no nul");
    // println!("cstring: {:?}", res_cstr);
    // let res_ptr = res_cstr.into_raw();
    res_cstr.into_raw()
}

fn reverse(src: &str) -> String {
    src.chars().rev().collect::<String>()
}
