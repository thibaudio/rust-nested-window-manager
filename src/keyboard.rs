use std::ptr::null_mut;

use native_windows_gui::{bind_raw_event_handler, ControlHandle, RawEventHandler};
use winapi::{shared::{windef::{HHOOK, HWND}, minwindef::{UINT, HIWORD, LOWORD}}, um::{winuser::{UnhookWindowsHookEx, SetWindowsHookExW, WH_KEYBOARD_LL, CallNextHookEx, RegisterHotKey, UnregisterHotKey, MOD_CONTROL, GetForegroundWindow}, errhandlingapi::GetLastError}};

use crate::window_manager::{self, WindowManager};

pub(crate) struct KeyboardEvent(isize);

pub(crate) struct KeyboardManager {
    window_handle: ControlHandle,
}

impl KeyboardManager {
    pub fn new(handle: ControlHandle) -> KeyboardManager {
        KeyboardManager {
            window_handle: handle,
        }
    }
    pub fn start_listening_for_keyboard_event(&mut self) {
        unsafe {
            RegisterHotKey(self.window_handle.hwnd().unwrap(), 1, MOD_CONTROL as UINT, 0x4B)
        };

    }

    pub fn stop_listening_for_keyboard_event(&mut self) {
        //let success = unsafe {UnhookWindowsHookEx(self.keyboard_hook)};
        let success = unsafe {UnregisterHotKey(self.window_handle.hwnd().unwrap(), 1)};
        if success == 0 {
            let error = unsafe { GetLastError() };
            println!("Unable to remove keyboard hook: {}", error);
        }
    }
}