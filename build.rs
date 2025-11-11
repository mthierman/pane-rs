extern crate embed_resource;
use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("TARGET")?.ends_with("windows-msvc") {
        let data = env::current_dir()?.join("data");
        let manifest_file = data.join("app.manifest").canonicalize()?;
        println!("cargo:rerun-if-changed={}", manifest_file.display());
        let icon_file = data.join("app.ico").canonicalize()?;
        println!("cargo:rerun-if-changed={}", icon_file.display());
        let resource_file = data.join("app.rc").canonicalize()?;
        println!("cargo:rerun-if-changed={}", resource_file.display());

        if !manifest_file.exists() {
            println!("cargo:warning={} not found", manifest_file.display());
        } else if let Some(path) = manifest_file.to_str() {
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", path);
        } else {
            println!(
                "cargo:warning=Manifest path is not valid UTF-8: {:?}",
                manifest_file
            );
        }

        let out_dir = PathBuf::from(env::var("OUT_DIR")?);

        //

        //

        // compile_resource("x64", resource_file, out_dir);
    }

    // let _ = embed_resource::compile("data/app.rc", embed_resource::NONE);
    // compile_resource(rc);

    Ok(())
}
