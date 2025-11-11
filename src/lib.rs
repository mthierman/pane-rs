use std::ffi::{OsStr, c_void};
use std::iter;
use std::os::windows::ffi::OsStrExt;
use std::{env, path::PathBuf, process::Command};
use windows::{
    Win32::{Foundation::HANDLE, UI::Shell::*},
    core::GUID,
};
use windows::{
    Win32::{
        Foundation::*, System::Diagnostics::Debug::OutputDebugStringW, System::LibraryLoader::*,
    },
    core::*,
};

pub fn debug_println(format_args: std::fmt::Arguments) {
    let s = format!("{format_args}\n");
    let wide: Vec<u16> = OsStr::new(&s).encode_wide().chain(iter::once(0)).collect();
    unsafe { OutputDebugStringW(PCWSTR(wide.as_ptr())) };
}

/// Convenience wrapper for `format!` syntax
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        $crate::debug_println(format_args!($($arg)*))
    };
}

// pub fn __debug_println_impl(s: &str) {
//     let wide: Vec<u16> = OsStr::new(s).encode_wide().chain(iter::once(0)).collect();
//     let ptr = PCWSTR(wide.as_ptr());

//     unsafe {
//         OutputDebugStringW(ptr);
//     }
// }

// #[macro_export]
// macro_rules! debug_println {
//     ($($arg:tt)*) => {{
//         $crate::__debug_println_impl(&format!($($arg)*));
//     }}
// }

pub fn get_instance() -> Result<HINSTANCE> {
    let mut hmodule = HMODULE::default();

    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            PCWSTR(get_instance as *const c_void as *const u16),
            &mut hmodule,
        )?;
    }

    Ok(hmodule.into())
}

pub fn known_folder(id: &GUID, flag: KNOWN_FOLDER_FLAG) -> PathBuf {
    PathBuf::from(unsafe {
        SHGetKnownFolderPath(id, flag, Some(HANDLE::default()))
            .unwrap()
            .to_string()
            .unwrap()
    })
}

pub fn current_dir() -> PathBuf {
    env::current_dir().unwrap()
}

pub fn out_dir() -> PathBuf {
    env::var("OUT_DIR").unwrap().into()
}

pub fn vswhere() -> PathBuf {
    known_folder(&FOLDERID_ProgramFilesX86, KF_FLAG_DONT_VERIFY)
        .join("Microsoft Visual Studio")
        .join("Installer")
        .join("vswhere.exe")
}

pub fn install_path() -> PathBuf {
    PathBuf::from(
        String::from_utf8(
            Command::new(vswhere())
                .args(["-property", "resolvedInstallationPath"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim(),
    )
}

pub fn winsdk_bat() -> PathBuf {
    install_path()
        .join("Common7")
        .join("Tools")
        .join("vsdevcmd")
        .join("core")
        .join("winsdk.bat")
}

pub fn windows_kit(arch: &str) -> PathBuf {
    let output = Command::new("cmd")
        .envs([("VSCMD_ARG_HOST_ARCH", arch), ("VSCMD_ARG_TGT_ARCH", arch)])
        .args([
            "/v:on",
            "/C",
            winsdk_bat().to_str().unwrap(),
            ">",
            "NUL",
            "&",
            "echo",
            "!WindowsSdkVerBinPath!",
        ])
        .output()
        .unwrap();

    PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
}

pub fn resource_compiler() -> PathBuf {
    windows_kit("x64").join("x64").join("rc.exe")
}

pub fn compile_resource(rc_file: PathBuf) {
    let rc = resource_compiler();

    if rc_file.exists() {
        // let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = out_dir();

        let res_file = out_dir.join(format!(
            "{}.res",
            rc_file.file_stem().unwrap().to_str().unwrap()
        ));

        Command::new(&rc)
            .args(["/fo", res_file.to_str().unwrap(), rc_file.to_str().unwrap()])
            .status()
            .unwrap();

        println!("cargo::rustc-link-arg-bins={}", res_file.to_str().unwrap());
    } else {
        println!("cargo:warning={} not found", rc_file.display());
    }
}

pub fn embed_manifest(path: PathBuf) {
    if !path.exists() {
        println!("cargo:warning={} not found", path.display());
    } else {
        println!("cargo::rustc-link-arg-bins=/MANIFEST:EMBED");
        println!(
            "cargo::rustc-link-arg-bins=/MANIFESTINPUT:{}",
            path.to_str().unwrap()
        );
    }
}
