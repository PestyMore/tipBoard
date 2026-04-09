
// This build script is used to embed the application icon and version metadata into the Windows executable.
fn main() {
    // We only need to run this for Windows targets
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
