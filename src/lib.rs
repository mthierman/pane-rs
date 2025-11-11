use std::ffi::{OsString, c_void};
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::{
    Win32::{Foundation::HANDLE, UI::Shell::*},
    core::GUID,
};
use windows::{
    Win32::{Foundation::*, System::LibraryLoader::*},
    core::*,
};

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

pub fn known_folder(id: &GUID, flag: Option<KNOWN_FOLDER_FLAG>) -> Result<PathBuf> {
    let known_folder = unsafe {
        SHGetKnownFolderPath(
            id,
            flag.unwrap_or(KNOWN_FOLDER_FLAG(0)),
            Some(HANDLE::default()),
        )?
    };

    Ok(PathBuf::from(OsString::from_wide(unsafe {
        known_folder.as_wide()
    })))
}
