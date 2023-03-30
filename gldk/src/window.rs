use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::platform_impl::window::RWindow;

#[repr(C)]
pub struct WindowID(pub u64);

#[repr(C)]
pub struct GLDKWindow {
    inner: RWindow
}

impl GLDKWindow {
    pub fn new(width:u32,height:u32,title: &str) -> Self {
        Self {
            inner: RWindow::new(width,height,title),
        }
    }

    pub fn id(&self) -> WindowID {
        self.inner.id()
    }

    pub fn run(&self) {
        self.inner.run();
    }
}

unsafe impl HasRawWindowHandle for GLDKWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}
