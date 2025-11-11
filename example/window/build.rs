use pane::resource_compiler;
use std::{env, error::Error, path::PathBuf, process::Command};

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

        let data_manifest = data.join("app.manifest").canonicalize()?;
        println!("cargo:rerun-if-changed={}", data_manifest.display());
        let data_icon = data.join("app.ico").canonicalize()?;
        println!("cargo:rerun-if-changed={}", data_icon.display());
        let data_resource = data.join("app.rc").canonicalize()?;
        println!("cargo:rerun-if-changed={}", data_resource.display());

        if !data_manifest.exists() {
            println!("cargo:warning={} not found", data_manifest.display());
        } else if let Some(path) = data_manifest.to_str() {
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", path);
        } else {
            println!(
                "cargo:warning=Manifest file is not valid UTF-8: {:?}",
                data_manifest
            );
        }

        if !data_resource.exists() {
            println!("cargo:warning={} not found", data_resource.display());
        } else if let Some(stem) = data_resource.file_stem() {
            let out_file = out_dir.join(stem).with_extension("res");

            Command::new(resource_compiler("x64")?)
                .args([
                    "/fo",
                    out_file.to_str().unwrap(),
                    data_resource.to_str().unwrap(),
                ])
                .status()?;

            println!("cargo::rustc-link-arg-bins={}", out_file.to_str().unwrap());
        } else {
            println!(
                "cargo:warning=Resource file is not valid UTF-8: {:?}",
                data_resource
            );
        }
    }

    Ok(())
}
