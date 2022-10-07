mod process;

extern crate native_windows_gui as nwg;

use nwg::{NativeUi, ControlHandle, Window};

pub struct ItemDefinition {
    shellexecute_path: String
}

pub struct Item {
    button_handle: nwg::Button,
    shellexecute_path: String
}

#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    buttons: Vec<Item>,
}

impl BasicApp {

    fn handle_button_click(&self, handle: &ControlHandle) {
        for button in &self.buttons {
            if handle == &button.button_handle {
                println!("Creating process: {}", button.shellexecute_path);
                match process::create_subprocess(&button.shellexecute_path) {
                    Err(error) => print!("Create process failed: {}", error),
                    _ => println!("Create process success") 
                };
            }
        }
    }
    
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use native_windows_gui as nwg;
    use nwg::bind_raw_event_handler;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }
    
    impl BasicApp {
        fn build_button(&mut self, item: ItemDefinition) -> Result<(), nwg::NwgError>{
            let index = self.buttons.len();
            let mut button_handle: nwg::Button = nwg::Button::default();
            nwg::Button::builder()
                .size((40, 40))
                .position((5 + (index * 40) as i32, 5))
                .text("G")
                .parent(&self.window)
                .build(&mut button_handle)?;
            self.buttons.push(Item{
                button_handle,
                shellexecute_path: item.shellexecute_path.clone()
            });
            Ok(())
        }
    } 

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;
            
            let width = 2560;
            let height: i32 = 1440; 
            let screen_size = 3440;
            let start_x = match screen_size > width {
                true => (screen_size - width)/2,
                false => 0
            };
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((width, 50))
                .position((start_x, 5))
                .title("Basic example")
                .build(&mut data.window)?;

            let item = ItemDefinition { shellexecute_path: "C:\\Godot\\Godot_v3.5-stable_win64.exe".to_string() };
            let item2 = ItemDefinition { shellexecute_path: "C:\\Users\\thiba\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe".to_string() };
            data.build_button(item);
            data.build_button(item2);

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => 
                            BasicApp::handle_button_click(&ui, &handle),
                        E::OnWindowClose => 
                            if &handle == &ui.window {
                                BasicApp::say_goodbye(&ui);
                            },
                        E::OnWindowMaximize =>
                            println!("Maximize"),
                        _ => {}
                    }
                }
            };

           *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    
    use winapi::um::winuser::WM_SIZE;
    use winapi::shared::minwindef::{HIWORD, LOWORD};
    const MY_UNIQUE_RAW_HANDLER_ID: usize = 457768;
    nwg::bind_raw_event_handler(&_ui.window.handle, MY_UNIQUE_RAW_HANDLER_ID, |_hnwd, msg, _w, l| {
        match msg {
            WM_MOVE => {
                println!("w: {}, h: {}", LOWORD(l as u32), HIWORD(l as u32));
            },
            _ => {}
        }
        None
    });
    nwg::dispatch_thread_events();
}