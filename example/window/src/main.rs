#![windows_subsystem = "windows"]
use pane::*;
use std::process::ExitCode;
use windows::{
    core::*,
    Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*},
};

fn main() -> Result<ExitCode> {
    let system_paths = SystemPaths::new()?;
    let json = serde_json::to_string_pretty(&system_paths).unwrap_or_default();
    println!("{}", json);

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
