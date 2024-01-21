mod error;
mod ffi;

use std::{ffi::CString, path::Path, process::Command};

pub use error::InjectError;
use ffi::inject_remote_process;

pub fn inject<S: AsRef<str>, P: AsRef<Path>>(
    pid: libc::pid_t,
    lib: P,
    func: Option<S>,
) -> Result<(), InjectError> {
    let enforce = Command::new("/system/bin/getenforce").output()?;
    let enforce = String::from_utf8_lossy(&enforce.stdout).into_owned().trim();

    handle_selinux(enforce)?;

    let lib = CString::new(lib.as_ref().to_str().unwrap())?;
    let func = func.as_ref().unwrap_or("symbols");
    let func = CString::new(func)?;
    let enforce = CString::new(enforce)?;
    unsafe {
        if inject_remote_process(pid, lib.as_ptr(), func.as_ptr(), enforce.as_ptr()) != 0 {
            return Err(InjectError::Failed);
        }
    }

    Ok(())
}

fn handle_selinux(enforce: &str) -> Result<(), InjectError> {
    if enforce.trim() == "Enforcing" {
        Command::new("/system/bin/setenforce")
            .arg("0")
            .spawn()?
            .wait()?;
    }

    Ok(())
}
