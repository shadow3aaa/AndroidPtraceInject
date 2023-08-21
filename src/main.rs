mod ffi;

use std::{
    env,
    ffi::CString,
    os::raw::{c_char, c_int},
};

fn main() {
    let args: Vec<_> = env::args().map(|arg| CString::new(arg).unwrap()).collect();

    let argc = args.len() as c_int;

    let mut c_args: Vec<_> = args.into_iter().map(|arg| arg.into_raw()).collect();
    let argv: *mut *mut c_char = c_args.as_mut_ptr();

    unsafe {
        ffi::inject_main(argc, argv);
    }
}
