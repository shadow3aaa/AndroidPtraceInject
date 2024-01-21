#[cfg(not(target_os = "android"))]
compile_error!("Only for android");
mod error;
mod ffi;

use std::{ffi::CString, path::Path, process::Command};

pub use error::InjectError;
use ffi::inject_remote_process;

/// pid: 目标进程的pid
/// lib: 要注入的so的路径
/// func: 注入后自动调用的函数的符号
pub fn inject<P: AsRef<Path>>(
    pid: libc::pid_t,
    lib: P,
    func: Option<&str>,
) -> Result<(), InjectError> {
    let enforce = Command::new("/system/bin/getenforce").output()?;
    let enforce = String::from_utf8_lossy(&enforce.stdout);
    let enforce = enforce.trim();

    preinject(enforce)?;

    let result = unsafe {
        let lib = CString::new(lib.as_ref().to_str().unwrap())?;
        let func = func.unwrap_or("symbols");
        let func = CString::new(func)?;
        let enforce = CString::new(enforce)?;

        if inject_remote_process(pid, lib.as_ptr(), func.as_ptr(), enforce.as_ptr()) != 0 {
            Err(InjectError::Failed)
        } else {
            Ok(())
        }
    };

    postinject(enforce)?;

    result
}

fn preinject(enforce: &str) -> Result<(), InjectError> {
    if enforce.trim() == "Enforcing" {
        Command::new("/system/bin/setenforce")
            .arg("0")
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn postinject(enforce: &str) -> Result<(), InjectError> {
    if enforce.trim() == "Enforcing" {
        Command::new("/system/bin/setenforce")
            .arg("1")
            .spawn()?
            .wait()?;
    }

    Ok(())
}
