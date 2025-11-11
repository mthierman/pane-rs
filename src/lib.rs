use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::{
    Win32::{
        Foundation::{HANDLE, HINSTANCE, HMODULE},
        System::LibraryLoader::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            GetModuleHandleExW,
        },
        UI::Shell::{KNOWN_FOLDER_FLAG, SHGetKnownFolderPath},
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
    let addr = get_instance as *const ();

    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            PCWSTR(addr.cast()),
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
