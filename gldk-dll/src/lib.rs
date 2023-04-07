#![no_main]

use gldk::window::GLDKWindow;
use gldk::GLVersion;
use std::ffi::{c_char, c_void, CStr};
use std::fmt::{Display, Formatter};
use std::sync::Mutex;
use once_cell::unsync::OnceCell;

static DOWNED_KEY:Mutex<OnceCell<u32>> = Mutex::new(OnceCell::new());
static UPPED_KEY:Mutex<OnceCell<u32>> = Mutex::new(OnceCell::new());

pub enum GLDKError {
    NullPtr,
    InvalidBool
}

#[repr(C)]
pub enum WindowEvent {
    RedrawRequested,
    Keydown,
    Keyup,
    CloseRequested
}

impl Into<WindowEvent> for gldk::window::WindowEvent {
    fn into(self) -> WindowEvent {
        match self {
            gldk::window::WindowEvent::RedrawRequested => WindowEvent::CloseRequested,
            gldk::window::WindowEvent::Keyup(_) => WindowEvent::Keyup,
            gldk::window::WindowEvent::Keydown(_) => WindowEvent::Keydown,
            gldk::window::WindowEvent::CloseRequested => WindowEvent::CloseRequested,
        }
    }
}

impl Display for GLDKError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GLDKError::NullPtr => write!(f, "NullPtr(00001): Invalid pointer passed."),
            GLDKError::InvalidBool => write!(f,"InvalidBool(00002): Invalid boolean type passed"),
        }
    }
}

fn panic_gldk(error: GLDKError) {
    println!();
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

    let b = Box::new(GLDKWindow::new(width, height, title, Some(config)).unwrap());
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn gldkRunWindow(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    window.run(|event| {
        match event {
            gldk::window::WindowEvent::Keyup(c) => {
                UPPED_KEY.lock().unwrap().set(c.0).unwrap()
            }
            gldk::window::WindowEvent::Keydown(c) => {
                DOWNED_KEY.lock().unwrap().set(c.0).unwrap()
            }
            gldk::window::WindowEvent::RedrawRequested => {
                match REDRAW_REQUESTED.lock().unwrap().get() {
                    None => {}
                    Some(c) => {
                        c();
                    }
                }
            }
            _ => {}
        }
    });
}

#[no_mangle]
pub extern "C" fn gldkShowWindow(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    window.show();
}

#[no_mangle]
pub extern "C" fn gldkQuitWindow(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    window.quit();
}

#[no_mangle]
pub extern "C" fn gldkHideWindow(window: *mut GLDKWindow) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };

    window.hide();
}

#[no_mangle]
pub extern "C" fn gldkSetUndecoratedWindow(window: *mut GLDKWindow,bool: u8) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    if bool >= 2 {
        panic_gldk(GLDKError::InvalidBool);
    }

    let window = unsafe { &*window };

    window.set_undecorated(bool != 0);
}

#[no_mangle]
pub extern "C" fn gldkSwapInterval(window: *mut GLDKWindow,bool: u8) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    if bool >= 2 {
        panic_gldk(GLDKError::InvalidBool);
    }

    let window = unsafe { &*window };

    window.swap_interval(bool != 0);
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
pub extern "C" fn gldkGetProcAddress(window: *mut GLDKWindow,s: *const c_char) -> *const c_void {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    let window = unsafe { &*window };
    unsafe {
        window.get_proc_address(CStr::from_ptr(s).to_str().unwrap())
    }
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

#[no_mangle]
pub extern "C" fn gldkSetWindowMinimized(window: *mut GLDKWindow,bool: u8) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    if bool >= 2 {
        panic_gldk(GLDKError::InvalidBool);
    }

    let window = unsafe { &*window };

    window.set_minimized(bool != 0);
}

#[no_mangle]
pub extern "C" fn gldkSetWindowMaximized(window: *mut GLDKWindow,bool: u8) {
    if window.is_null() {
        panic_gldk(GLDKError::NullPtr);
    }

    if bool >= 2 {
        panic_gldk(GLDKError::InvalidBool);
    }

    let window = unsafe { &*window };

    window.set_maximized(bool != 0);
}

// Callbacks
pub type REDRAWREQUESTEDCALLBACK = extern "C" fn();
pub type CLOSEREQUESTEDCALLBACK = extern "C" fn();
pub type KEYDOWNEDCALLBACK = extern "C" fn(u32);
pub type KEYUPPEDCALLBACK = extern "C" fn(u32);

static REDRAW_REQUESTED:Mutex<OnceCell<REDRAWREQUESTEDCALLBACK>> = Mutex::new(OnceCell::new());
static CLOSE_REQUESTED:Mutex<OnceCell<CLOSEREQUESTEDCALLBACK>> = Mutex::new(OnceCell::new());
static KEY_DOWNED:Mutex<OnceCell<KEYDOWNEDCALLBACK>> = Mutex::new(OnceCell::new());
static KEY_UPPED:Mutex<OnceCell<KEYUPPEDCALLBACK>> = Mutex::new(OnceCell::new());

#[no_mangle]
pub extern "C" fn gldkSetRedrawRequestedCallback(callback: REDRAWREQUESTEDCALLBACK) {
    REDRAW_REQUESTED.lock().unwrap().set(callback).unwrap();
}

#[no_mangle]
pub extern "C" fn gldkSetCloseRequestedCallback(callback: CLOSEREQUESTEDCALLBACK) {
    CLOSE_REQUESTED.lock().unwrap().set(callback).unwrap();
}

#[no_mangle]
pub extern "C" fn gldkSetKeyDownedCallBack(callback: KEYDOWNEDCALLBACK) {
    KEY_DOWNED.lock().unwrap().set(callback).unwrap();
}
#[no_mangle]
pub extern "C" fn gldkSetKeyUppedCallBack(callback: KEYUPPEDCALLBACK) {
    KEY_DOWNED.lock().unwrap().set(callback).unwrap();
}
