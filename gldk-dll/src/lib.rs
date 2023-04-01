#![no_main]

use gldk::window::{GLDKWindow, WindowEvent};
use gldk::GLVersion;
use std::ffi::{c_char, CStr};
use std::fmt::{Arguments, Display, Formatter};

pub enum GLDKError {
    NullPtr,
}

impl Display for GLDKError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GLDKError::NullPtr => write!(f, "NullPtr(00001): Invalid pointer passed."),
        }
    }
}

fn panic_gldk(error: GLDKError) {
    println!("GLDK error {}", error);
    std::process::exit(1);
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
                panic_gldk(GLDKError::NullPtr);
                std::process::exit(1);
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
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    window.run(|event| {
        callback(event);
    });
}

#[no_mangle]
pub extern "C" fn gldkMakeCurrent(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };
    window.make_current()
}

#[no_mangle]
pub extern "C" fn gldkSwapBuffers(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };
    window.swap_buffers();
}

#[no_mangle]
pub extern "C" fn gldkSetWindowTitle(window: *mut GLDKWindow, title: *const c_char) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    window.set_window_title(unsafe { CStr::from_ptr(title).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn gldkGetWindowSize(window: *mut GLDKWindow, width: &mut u32, height: &mut u32) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    let (w, h) = window.get_window_size();
    *width = w;
    *height = h;
}

#[no_mangle]
pub extern "C" fn gldkGetWindowPos(window: *mut GLDKWindow, x_ptr: &mut u32, y_ptr: &mut u32) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    let (x, y) = window.get_window_pos();
    *x_ptr = x;
    *y_ptr = y;
}
