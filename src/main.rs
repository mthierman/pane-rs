#![windows_subsystem = "windows"]
use pane_rs::*;
use std::process::ExitCode;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};

fn main() -> Result<ExitCode> {
    let folder = known_folder(FOLDERID_LocalAppData, None)?;
    println!("{}", folder.to_str().unwrap());

    let vswhere = vswhere()?;
    println!("{}", vswhere.to_str().unwrap());

    let install_path = install_path()?;
    println!("{}", install_path.to_str().unwrap());

    let winsdk_bat = winsdk_bat()?;
    println!("{}", winsdk_bat.to_str().unwrap());

    let windows_kit = windows_kit("x64")?;
    println!("{}", windows_kit.to_str().unwrap());

    let rc = resource_compiler("x64")?;
    println!("{}", rc.to_str().unwrap());

    let wc = WNDCLASSEXW {
        lpszClassName: w!("window"),
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap() },
        hInstance: get_instance().unwrap(),
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
        )
        .unwrap();
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
