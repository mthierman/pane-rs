extern crate embed_resource;
use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    // let data = env::current_dir()?.join("data");
    // let rc = data.join("app.rc");
    // let manifest = data.join("app.manifest");
    // let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    if env::var("TARGET")?.ends_with("windows-msvc") {
        let manifest = Path::new("data/app.manifest").canonicalize().unwrap();
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!(
            "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
            manifest.display()
        );
        println!("cargo:rerun-if-changed=data/app.manifest");
    }

    println!("cargo:rerun-if-changed=build.rs");
    let _ = embed_resource::compile("data/app.rc", embed_resource::NONE);

    // compile_resource(rc);
    // embed_manifest(manifest);

    Ok(())
}
