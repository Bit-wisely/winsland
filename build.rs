fn main() {
    // Embed the manifest (per-monitor DPI v2) into the exe on Windows builds.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("winisland.manifest");
        // res.set_icon("assets/icons/app.ico"); // enable once the icon exists
        if let Err(e) = res.compile() {
            println!("cargo:warning=resource compile failed: {e}");
        }
    }
    println!("cargo:rerun-if-changed=winisland.manifest");
}
