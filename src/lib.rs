use std::ffi::c_void;
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
