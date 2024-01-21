use anyhow::Result;

#[cfg(not(target_os = "android"))]
compile_error!("Only for android");

fn main() -> Result<()> {
    cc::Build::new()
        .file("Inject/PtraceInject.c")
        .warnings(false)
        .include("Inject/Utils")
        .compile("inject");
    Ok(())
}
