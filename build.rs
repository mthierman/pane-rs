// use std::env;
// extern crate embed_resource;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Old one
    // if env::var("TARGET")
    //     .expect("target")
    //     .ends_with("windows-msvc")
    // {
    //     let manifest = Path::new("data/app.manifest").canonicalize().unwrap();
    //     println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    //     println!(
    //         "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
    //         manifest.display()
    //     );
    //     println!("cargo:rerun-if-changed=data/app.manifest");
    // }
    // println!("cargo:rerun-if-changed=build.rs");
    // let _ = embed_resource::compile("data/app.rc", embed_resource::NONE);

    // New one
    // let data = env::current_dir()?.join("data");
    // let rc = data.join("app.rc");
    // tools::compile_resource(rc);
    // let manifest = data.join("app.manifest");
    // tools::embed_manifest(manifest);

    Ok(())
}
