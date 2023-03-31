use crate::platform_impl::window::RWindow;
use crate::GLConfig;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

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

    pub fn run(&self)
    {
        self.inner.run();
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
