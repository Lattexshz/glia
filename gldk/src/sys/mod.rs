use crate::sys::wgl::types::GLenum;
use std::ffi::{c_char, CStr};

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[link(name = "Opengl32")]
extern "system" {
    pub fn glGetString(name: u32) -> *mut u8;
    pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) -> ();
    pub fn glClear(mask: u32) -> ();
}

#[repr(C)]
pub struct GL {
    major: GLenum,
    null: GLenum,
    minor: GLenum,
}

impl GL {
    pub fn new(major: GLenum, minor: GLenum) -> Self {
        Self {
            major,
            null: 0,
            minor,
        }
    }

    pub fn major_version(&self) -> GLenum {
        self.major
    }

    pub fn minor_version(&self) -> GLenum {
        self.minor
    }

    pub fn get_string(&self, _name: u32) -> &'static str {
        unsafe {
            let val = glGetString(gl::VERSION);
            let cstr = CStr::from_ptr(val as *const c_char);
            cstr.to_str().unwrap()
        }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            glClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear(&self, mask: u32) {
        unsafe {
            glClear(mask);
        }
    }
}
