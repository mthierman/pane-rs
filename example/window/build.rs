use pane::*;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rustc-link-arg-bins=/WX");

    if env::var("TARGET").unwrap().ends_with("windows-msvc") {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let data = root.join("data");

        if !data.exists() {
            println!("cargo:warning={} not found", data.display());
        }

        embed_manifest(&data.join("app.manifest")).unwrap();
        compile_resource(
            "x64",
            &data.join("app.rc"),
            &PathBuf::from(env::var("OUT_DIR").unwrap()),
        )
        .unwrap();
    }
}
