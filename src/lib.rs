use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::{ffi::OsString, os::windows::process::CommandExt};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Threading::CREATE_NO_WINDOW;
use windows::Win32::UI::WindowsAndMessaging::TranslateMessage;
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
        UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG},
    },
    core::{Error, GUID, PCWSTR, PWSTR, Result},
};

trait PwstrExt {
    fn to_pathbuf(&self) -> PathBuf;
}

impl PwstrExt for PWSTR {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from(OsString::from_wide(unsafe { self.as_wide() }))
    }
}

pub fn message_loop() -> ExitCode {
    let mut msg = MSG::default();

    loop {
        let result = unsafe { GetMessageW(&mut msg, None, 0, 0) };

        match result.0 {
            -1 => return ExitCode::FAILURE,
            0 => break,
            _ => unsafe {
                let _ = TranslateMessage(&msg);
                let _ = DispatchMessageW(&msg);
            },
        }
    }

    ExitCode::from(msg.wParam.0.try_into().unwrap_or(0))
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
    Ok(PathBuf::from(
        String::from_utf8(
            Command::new(vswhere()?)
                .creation_flags(CREATE_NO_WINDOW.0)
                .args(["-property", "resolvedInstallationPath"])
                .output()?
                .stdout,
        )?
        .trim(),
    ))
}

pub fn winsdk_bat() -> Result<PathBuf> {
    let mut buffer = install_path()?;
    let components = ["Common7", "Tools", "vsdevcmd", "core", "winsdk.bat"];

    components.iter().for_each(|c| buffer.push(c));

    Ok(buffer)
}

pub fn windows_kit(arch: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(
        String::from_utf8(
            Command::new("cmd")
                .creation_flags(CREATE_NO_WINDOW.0)
                .envs([("VSCMD_ARG_HOST_ARCH", arch), ("VSCMD_ARG_TGT_ARCH", arch)])
                .args([
                    "/v:on",
                    "/C",
                    winsdk_bat()?.to_str().unwrap(),
                    ">",
                    "NUL",
                    "&",
                    "echo",
                    "!WindowsSdkVerBinPath!",
                ])
                .output()?
                .stdout,
        )?
        .trim()
        .trim_end_matches(['\\', '/']),
    ))
}

pub fn resource_compiler(arch: &str) -> Result<PathBuf> {
    let mut buffer = windows_kit(arch)?;
    let components = [arch, "rc.exe"];

    components.iter().for_each(|c| buffer.push(c));

    Ok(buffer)
}

pub fn compile_resource(arch: &str, rc_file: PathBuf, out_dir: PathBuf) -> Result<()> {
    let rc = resource_compiler(arch)?;

    if rc_file.exists() {
        let res_file = out_dir.join(format!(
            "{}.res",
            rc_file
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        ));

        Command::new(&rc)
            .args([
                "/fo",
                res_file.to_str().unwrap_or_default(),
                rc_file.to_str().unwrap_or_default(),
            ])
            .status()
            .unwrap();

        println!(
            "cargo::rustc-link-arg-bins={}",
            res_file.to_str().unwrap_or_default()
        );
    } else {
        println!("cargo:warning={} not found", rc_file.display());
    }

    Ok(())
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
