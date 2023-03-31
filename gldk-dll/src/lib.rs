#![no_main]

use gldk::window::GLDKWindow;
use gldk::GLVersion;
use std::ffi::{c_char, CStr};
use std::str::Utf8Error;

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

impl Into<gldk::GLConfig> for GLConfig {
    fn into(self) -> gldk::GLConfig {
        gldk::GLConfig {
            version: self.version,
        }
    }
}

#[no_mangle]
pub extern "C" fn gldkCreateWindow(
    width: u32,
    height: u32,
    title: *const c_char,
    config: *const GLConfig,
) -> *mut GLDKWindow {
    let config = if config.is_null() {
        GLConfig::default()
    } else {
        unsafe { &*config }
    };

    let title = unsafe {
        match CStr::from_ptr(title).to_str() {
            Ok(t) => t,
            Err(_) => {
                panic_gldk!();
            }
        }
    };

    let b = Box::new(GLDKWindow::new(width, height, title, Some(config.into())));
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn gldkShowWindow(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk!();
    }

    let window = unsafe { &*window };
    window.run(|| {});
}
