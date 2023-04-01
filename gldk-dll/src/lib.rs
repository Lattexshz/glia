#![no_main]

use gldk::window::{GLDKWindow, WindowEvent};
use gldk::GLVersion;
use std::ffi::{c_char, CStr};

macro_rules! panic_gldk {
    () => {
        println!("GLDK ERROR");
        std::process::exit(1);
    };
}

#[repr(C)]
pub struct GLConfig {
    version: GLVersion,
}

impl Default for GLConfig {
    fn default() -> Self {
        Self {
            version: GLVersion::V3_0,
        }
    }
}

#[no_mangle]
pub extern "C" fn gldkCreateWindow(
    width: u32,
    height: u32,
    title: *const c_char,
    config: GLConfig,
) -> *mut GLDKWindow {
    let title = unsafe {
        match CStr::from_ptr(title).to_str() {
            Ok(t) => t,
            Err(_) => {
                panic_gldk!();
            }
        }
    };

    let config = gldk::GLConfig {
        version: config.version,
    };

    let b = Box::new(GLDKWindow::new(width, height, title, Some(config)));
    Box::into_raw(b)
}

pub type CALLBACKPROC = extern "C" fn(WindowEvent);

#[no_mangle]
pub extern "C" fn gldkShowWindow(window: *mut GLDKWindow, callback: CALLBACKPROC) {
    if window.is_null() {
        panic_gldk!();
    }

    let window = unsafe { &*window };

    window.run(|event| {
        callback(event);
    });
}

#[no_mangle]
pub extern "C" fn gldkMakeCurrent(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk!();
    }

    let window = unsafe { &*window };
    window.make_current()
}


#[no_mangle]
pub extern "C" fn gldkSwapBuffers(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk!();
    }

    let window = unsafe { &*window };
    window.swap_buffers();
}
