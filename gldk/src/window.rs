use crate::platform_impl::window::RWindow;
use crate::GLConfig;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;
use crate::error::GLDKError;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlFlow {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowEvent {
    RedrawRequested,

    Keyup(KeyCode),
    Keydown(KeyCode),

    CloseRequested,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyCode(pub u32);

impl Into<char> for KeyCode {
    fn into(self) -> char {
        std::char::from_u32(self.0).unwrap()
    }
}

#[repr(C)]
pub struct WindowID(pub u64);

#[repr(C)]
pub struct GLDKWindow {
    inner: RWindow,
}

impl GLDKWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: Option<GLConfig>) -> Result<Self,GLDKError> {
        let conf = match conf {
            None => GLConfig::default(),
            Some(c) => c,
        };

        let inner = match RWindow::new(width, height, title, conf) {
            Ok(i) => i,
            Err(e) => return Err(e)
        };

        Ok(Self {
            inner
        })
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

    pub fn swap_interval(&self,enable: bool) {
        self.inner.swap_interval(enable);
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
