use pane::*;
use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rustc-link-arg-bins=/WX");

    if env::var("TARGET")?.ends_with("windows-msvc") {
        let out_dir = PathBuf::from(env::var("OUT_DIR")?);
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let data = manifest_dir.join("data");

        let data_icon = data.join("app.ico").canonicalize()?;
        println!("cargo:rerun-if-changed={}", data_icon.display());
        let data_resource = data.join("app.rc").canonicalize()?;
        println!("cargo:rerun-if-changed={}", data_resource.display());

        let data_manifest = data.join("app.manifest").canonicalize()?;
        println!("cargo:rerun-if-changed={}", data_manifest.display());

        embed_manifest(&data_manifest)?;
        compile_resource("x64", &data_resource, &out_dir)?;
    }

    Ok(())
}
