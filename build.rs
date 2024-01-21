use anyhow::Result;

fn main() -> Result<()> {
    cc::Build::new()
        .file("Inject/PtraceInject.c")
        .include("Inject/Utils")
        .compile("inject");
    Ok(())
}
