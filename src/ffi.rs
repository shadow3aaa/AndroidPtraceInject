extern "C" {
    pub fn inject_remote_process(
        pid: libc::pid_t,
        lib_path: *const libc::c_char,
        function_name: *const libc::c_char,
        selinux_flag: *const lib::c_char,
    ) -> ::std::os::raw::c_int;
}
