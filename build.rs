use std::{env, path::Path};
extern crate embed_resource;

fn main() {
    if env::var("TARGET")
        .expect("target")
        .ends_with("windows-msvc")
    {
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
}
