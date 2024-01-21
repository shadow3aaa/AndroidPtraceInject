use std::env::consts::OS;

use anyhow::Result;

#[cfg(not(target_os = "android"))]
compile_error!("Only for android");

fn main() -> Result<()> {
    if OS == "windows" {
        println!("cargo:rustc-link-search=native=prebuilt");
        println!("cargo:rustc-link-lib=static=prebuilt");
    } else {
        cc::Build::new()
            .file("Inject/PtraceInject.c")
            .warnings(false)
            .include("Inject/Utils")
            .compile("inject");
    }
    Ok(())
}
