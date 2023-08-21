use anyhow::Result;

fn main() -> Result<()> {
    cc::Build::new()
        .file("Inject/inject.cpp")
        .include("Inject/include")
        .include("Inject/include/Utils")
        .cpp(true)
        .compile("inject");
    Ok(())
}
