use crate::platform_impl::window::RWindow;
use crate::GLConfig;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowEvent {
    Update,

    Keyup(KeyCode),
    Keydown(KeyCode),

    CloseRequested,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyCode(pub u32);

pub trait CallBack {
    fn update(&self);
}

#[repr(C)]
pub struct WindowID(pub u64);

#[repr(C)]
pub struct GLDKWindow {
    inner: RWindow,
}

impl GLDKWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: Option<GLConfig>) -> Self {
        let conf = match conf {
            None => GLConfig::default(),
            Some(c) => c,
        };

        Self {
            inner: RWindow::new(width, height, title, conf),
        }
    }

    pub fn id(&self) -> WindowID {
        self.inner.id()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        self.inner.get_proc_address(addr)
    }

    pub fn run<F>(&self, callback: F)
    where
        F: Fn(WindowEvent),
    {
        self.inner.run(callback);
    }

    pub fn make_current(&self) {
        self.inner.make_current();
    }

    pub fn swap_buffers(&self) {
        self.inner.swap_buffers();
    }
}

unsafe impl HasRawWindowHandle for GLDKWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}
