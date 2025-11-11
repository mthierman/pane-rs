use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::process::Command;
use windows::{
    Win32::{
        Foundation::{HANDLE, HINSTANCE, HMODULE},
        System::LibraryLoader::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            GetModuleHandleExW,
        },
        UI::Shell::{
            FOLDERID_ProgramFilesX86, KF_FLAG_DONT_VERIFY, KNOWN_FOLDER_FLAG, SHGetKnownFolderPath,
        },
    },
    core::{GUID, PCWSTR, PWSTR, Result},
};

trait PwstrExt {
    fn to_pathbuf(&self) -> PathBuf;
}

impl PwstrExt for PWSTR {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from(OsString::from_wide(unsafe { self.as_wide() }))
    }
}

pub fn get_instance() -> Result<HINSTANCE> {
    let mut hmodule = HMODULE::default();

    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            PCWSTR((get_instance as *const ()).cast()),
            &mut hmodule,
        )?
    }

    Ok(hmodule.into())
}

pub fn known_folder(id: GUID, flag: Option<KNOWN_FOLDER_FLAG>) -> Result<PathBuf> {
    let buffer = unsafe {
        SHGetKnownFolderPath(
            &id,
            flag.unwrap_or(KNOWN_FOLDER_FLAG(0)),
            Some(HANDLE::default()),
        )?
    };

    Ok(buffer.to_pathbuf())
}

pub fn vswhere() -> Result<PathBuf> {
    let mut buffer = known_folder(FOLDERID_ProgramFilesX86, Some(KF_FLAG_DONT_VERIFY))?;
    let components = ["Microsoft Visual Studio", "Installer", "vswhere.exe"];

    components.iter().for_each(|c| buffer.push(c));

    Ok(buffer)
}

pub fn install_path() -> Result<PathBuf> {
    let result = Command::new(vswhere()?)
        .args(["-property", "resolvedInstallationPath"])
        .output()?
        .stdout;

    Ok(PathBuf::from(String::from_utf8(result)?.trim()))
}

// pub fn install_path() -> PathBuf {
//     PathBuf::from(
//         String::from_utf8(
//             Command::new(vswhere())
//                 .args(["-property", "resolvedInstallationPath"])
//                 .output()
//                 .unwrap()
//                 .stdout,
//         )
//         .unwrap()
//         .trim(),
//     )
// }

// pub fn winsdk_bat() -> PathBuf {
//     install_path()
//         .join("Common7")
//         .join("Tools")
//         .join("vsdevcmd")
//         .join("core")
//         .join("winsdk.bat")
// }

// pub fn windows_kit(arch: &str) -> PathBuf {
//     let output = Command::new("cmd")
//         .envs([("VSCMD_ARG_HOST_ARCH", arch), ("VSCMD_ARG_TGT_ARCH", arch)])
//         .args([
//             "/v:on",
//             "/C",
//             winsdk_bat().to_str().unwrap(),
//             ">",
//             "NUL",
//             "&",
//             "echo",
//             "!WindowsSdkVerBinPath!",
//         ])
//         .output()
//         .unwrap();

//     PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
// }

// pub fn resource_compiler() -> PathBuf {
//     windows_kit("x64").join("x64").join("rc.exe")
// }

// pub fn compile_resource(rc_file: PathBuf) {
//     let rc = resource_compiler();

//     if rc_file.exists() {
//         // let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
//         let out_dir = out_dir();

//         let res_file = out_dir.join(format!(
//             "{}.res",
//             rc_file.file_stem().unwrap().to_str().unwrap()
//         ));

//         Command::new(&rc)
//             .args(["/fo", res_file.to_str().unwrap(), rc_file.to_str().unwrap()])
//             .status()
//             .unwrap();

//         println!("cargo::rustc-link-arg-bins={}", res_file.to_str().unwrap());
//     } else {
//         println!("cargo:warning={} not found", rc_file.display());
//     }
// }

// pub fn embed_manifest(path: PathBuf) {
//     if !path.exists() {
//         println!("cargo:warning={} not found", path.display());
//     } else {
//         println!("cargo::rustc-link-arg-bins=/MANIFEST:EMBED");
//         println!(
//             "cargo::rustc-link-arg-bins=/MANIFESTINPUT:{}",
//             path.to_str().unwrap()
//         );
//     }
// }
