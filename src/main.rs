use pane_rs::*;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};

fn main() -> Result<()> {
    let folder = known_folder(&FOLDERID_LocalAppData, None);

    println!("{}", folder.unwrap().to_str().unwrap());

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

    let mut msg = MSG::default();

    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).into() {
            DispatchMessageW(&msg);
        }
    }

    Ok(())
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
