#![no_main]

use gldk::window::GLDKWindow;

#[no_mangle]
pub extern "C" fn gldkCreateWindow(width: u32,height: u32,title: &str) -> *mut GLDKWindow {
    let b = Box::new(GLDKWindow::new(width,height,title));
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn gldkShowWindow(window: *mut GLDKWindow) {
    let window = unsafe { &*window };
    window.run();
}