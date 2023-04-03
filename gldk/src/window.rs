use crate::platform_impl::window::RWindow;
use crate::GLConfig;
use gwl::window::{IWindow};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlFlow {}

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
        F: FnMut(WindowEvent),
    {
        self.inner.run(callback);
    }

    // GL Functions

    pub fn make_current(&self) {
        self.inner.make_current();
    }

    pub fn swap_buffers(&self) {
        self.inner.swap_buffers();
    }

    // Common

    pub fn show(&self) {
        self.inner.show();
    }

    pub fn hide(&self) {
        self.inner.hide();
    }

    pub fn set_window_title(&self, title: &str) {
        self.inner.set_window_title(title);
    }

    pub fn set_window_border_width(&self, width: u32) {
        self.inner.set_window_border_width(width);
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.inner.get_window_size()
    }

    pub fn get_window_pos(&self) -> (u32, u32) {
        self.inner.get_window_pos()
    }

    pub fn set_undecorated(&self,b: bool) {
        self.inner.set_undecorated(b);
    }
}

unsafe impl HasRawWindowHandle for GLDKWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}
