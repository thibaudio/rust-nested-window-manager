use std::{thread, sync::{Arc, Mutex}};

use winapi::{shared::{minwindef::LPARAM, ntdef::LPWSTR, windef::{HWND}}, um::{winuser::{SetWindowPos, SWP_FRAMECHANGED, FindWindowW, EnumWindows}}};

struct Window(HWND);

unsafe impl Send for Window {}

pub(crate) struct WindowManager {
    watched_windows: Arc<Mutex<Vec<Window>>>,
}

impl WindowManager {
    pub fn new() -> WindowManager {
        WindowManager {  
           watched_windows: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn start(&self) {
        let mut watched_windows = self.watched_windows.clone();
        thread::spawn(move || loop {
            {
                let mut watched_windows_in_thread = watched_windows.lock().unwrap();
                for window in watched_windows_in_thread.iter_mut() {
                    unsafe {SetWindowPos(window.0, 0 as HWND, ((3440 - 2560) / 2) as i32, 0, 2560, 1440, SWP_FRAMECHANGED);}
                }
            }
            thread::sleep(std::time::Duration::from_secs(1));
        });
    }

    pub fn register_application(&self, hwnd: HWND) {
        let mut watched_windows = self.watched_windows.clone();
        let mut watched_windows_in_thread = watched_windows.lock().unwrap();
        watched_windows_in_thread.push(Window(hwnd));
    }
}

pub fn get_applications() -> Option<HWND> {
    unsafe { 
        let mut hwnd = FindWindowW(
            0 as LPWSTR,
            "something else to enter hwnd.is_null()".encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
        );
        EnumWindows(Some(enum_window), &mut hwnd as *mut HWND as LPARAM);
        if hwnd.is_null() {
           None 
        }
        else {
            Some(hwnd)
        }
    }
}

extern "system" fn enum_window(hwnd: HWND, l_param: LPARAM) -> i32 {
    // --- std ---
    use std::{
        ffi::OsString,
        os::windows::ffi::OsStringExt,
    };
    // --- external ---
    use winapi::um::winuser::GetWindowTextW;
    unsafe {
        let mut buf = [0; 128];
        let len = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        let name = OsString::from_wide(&buf[..len as usize]);
        if name.to_str().unwrap().contains("Godot") {
            *(l_param as *mut HWND) = hwnd;
            0
        } else { 1 }
    }
}