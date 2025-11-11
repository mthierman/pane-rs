#![windows_subsystem = "windows"]
use pane_rs::*;
use serde::Serialize;
use std::{path::PathBuf, process::ExitCode};
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};

#[derive(Serialize)]
struct Paths {
    local_app_data: PathBuf,
    vswhere: PathBuf,
    install_path: PathBuf,
    winsdk_bat: PathBuf,
    windows_kit: PathBuf,
    resource_compiler: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        Ok(Self {
            local_app_data: known_folder(FOLDERID_LocalAppData, None)?,
            vswhere: vswhere()?,
            install_path: install_path()?,
            winsdk_bat: winsdk_bat()?,
            windows_kit: windows_kit("x64")?,
            resource_compiler: resource_compiler("x64")?,
        })
    }
}

fn main() -> Result<ExitCode> {
    let paths = Paths::new()?;

    let json = serde_json::to_string_pretty(&paths).unwrap();
    println!("{}", json);

    // println!("{}", paths.local_app_data.to_str().unwrap());
    // println!("{}", vswhere.to_str().unwrap());
    // println!("{}", install_path.to_str().unwrap());
    // println!("{}", winsdk_bat.to_str().unwrap());
    // println!("{}", windows_kit.to_str().unwrap());
    // println!("{}", rc.to_str().unwrap());

    let wc = WNDCLASSEXW {
        lpszClassName: w!("window"),
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
        hInstance: get_instance()?,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };

    let atom = unsafe { RegisterClassExW(&wc) };
    debug_assert!(atom != 0);

    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            wc.lpszClassName,
            wc.lpszClassName,
            WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            get_instance().ok(),
            None,
        )?;
    }

    message_loop()
}

extern "system" fn wndproc(window: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_PAINT => {
                println!("WM_PAINT");
                _ = ValidateRect(Some(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, msg, wparam, lparam),
        }
    }
}
