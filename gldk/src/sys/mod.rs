use crate::sys::wgl::types::GLenum;
use std::ffi::{c_char, CStr};
use windows_sys::Win32::Graphics::OpenGL::GL_PROJECTION;

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
    pub fn glMatrixMode(mode: GLenum);
    pub fn glLoadIdentity();
    pub fn glBegin(mode: GLenum) -> ();
    pub fn glGetString(name: u32) -> *mut u8;
    pub fn glColor3f(red: f32, green: f32, blue: f32);
    pub fn glVertex3f(red: f32, green: f32, blue: f32);
    pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) -> ();
    pub fn glClear(mask: u32) -> ();
    pub fn glEnd() -> ();
    pub fn glViewport(x: u32,y: u32,width: u32,height: u32);
}

#[repr(C)]
pub struct GL {
    major: GLenum,
    minor: GLenum,
}

impl GL {
    pub fn new(major: GLenum, minor: GLenum) -> Self {
        unsafe {
            glViewport(0,0,500,500);
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
        }
        Self {
            major,
            minor,
        }
    }

    pub fn major_version(&self) -> GLenum {
        self.major
    }

    pub fn minor_version(&self) -> GLenum {
        self.minor
    }

    pub fn begin(&self,mode: GLenum) {
        unsafe {
            glBegin(mode);
        }
    }

    pub fn get_string(&self, name: u32) -> &'static str {
        unsafe {
            let val = glGetString(name);
            let cstr = CStr::from_ptr(val as *const c_char);
            cstr.to_str().unwrap()
        }
    }

    pub fn color3f(&self, red: f32, green: f32, blue: f32) {
        unsafe {
            glColor3f(red,green,blue);
        }
    }

    pub fn vertex3f(&self, red: f32, green: f32, blue: f32) {
        unsafe {
            glVertex3f(red,green,blue);
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

    pub fn end(&self) {
        unsafe {
            glEnd();
        }
    }
}
