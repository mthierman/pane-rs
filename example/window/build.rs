use pane::*;
use std::{env, path::PathBuf};
use windows::{
    Win32::Foundation::ERROR_FILE_NOT_FOUND,
    core::{Error, Result},
};

fn main() -> Result<()> {
    if env::var("TARGET").unwrap().ends_with("windows-msvc") {
        println!("cargo::rustc-link-arg-bins=/WX");

        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let data = root.join("data");

        if !data.exists() {
            println!("cargo:warning={} not found", data.display());
            return Err(Error::new(
                ERROR_FILE_NOT_FOUND.to_hresult(),
                "File not found",
            ));
        }

        embed_manifest(&data.join("app.manifest"))?;

        compile_resource(
            &data.join("app.rc"),
            &PathBuf::from(env::var("OUT_DIR").unwrap()),
        )?;
    }

    Ok(())
}
