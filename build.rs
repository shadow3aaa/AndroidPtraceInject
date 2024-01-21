use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=native=prebuilt");
        println!("cargo:rustc-link-lib=static=prebuilt");
    }

    #[cfg(not(target_os = "windows"))]
    {
        cc::Build::new()
            .file("Inject/PtraceInject.c")
            .warnings(false)
            .include("Inject/Utils")
            .compile("inject");
    }

    Ok(())
}
