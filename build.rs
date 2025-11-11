extern crate embed_resource;
use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("TARGET")?.ends_with("windows-msvc") {
        let manifest = env::current_dir()?
            .join("data")
            .join("app.manifest")
            .canonicalize()?;

        let icon = env::current_dir()?
            .join("data")
            .join("app.ico")
            .canonicalize()?;

        let resource_file = env::current_dir()?
            .join("data")
            .join("app.rc")
            .canonicalize()?;

        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rerun-if-changed={}", icon.display());
        println!("cargo:rerun-if-changed={}", resource_file.display());

        if !manifest.exists() {
            println!("cargo:warning={} not found", manifest.display());
        } else if let Some(path) = manifest.to_str() {
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", path);
        } else {
            println!(
                "cargo:warning=Manifest path is not valid UTF-8: {:?}",
                manifest
            );
        }

        let out_dir = PathBuf::from(env::var("OUT_DIR")?);
        let resource_file = data.join("app.rc").canonicalize()?;
    }
    // let _ = embed_resource::compile("data/app.rc", embed_resource::NONE);
    // compile_resource(rc);

    Ok(())
}
