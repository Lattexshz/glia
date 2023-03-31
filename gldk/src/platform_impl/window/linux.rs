use crate::window::WindowID;
use raw_window_handle::{RawWindowHandle, XlibWindowHandle};
use safex::glx::*;
use safex::xlib::*;
use crate::GLConfig;
use core::ffi::c_void;

pub struct RWindow {
    display: Display,
    window: Window,
    glc: GLXContext,
    cmap: ColorMap,
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str,conf: GLConfig) -> Self {
        let display = Display::open(None);
        let screen = Screen::default(&display);
        let root = Window::root_window(&display, &screen);

        let cmap = ColorMap::default(&display, &screen);

        let vi = glx_choose_visual(
            &display,
            &mut [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_NONE],
        )
        .unwrap();
        let window = Window::new_with_glx(
            &display, &screen, &vi, None, 0, 0, width, height, 1, vi.depth, 0, &vi,
        )
        .unwrap();

        let glc = GLXContext::create(&display, &vi, None, gl::TRUE as i32);
        glx_make_current(&display, &window, &glc);

        window.map(&display);
        window.set_window_title(title);

        Self {
            display,
            window,
            glc,
            cmap,
        }
    }

    pub fn get_proc_address(&self,addr: &str) -> *const c_void {
        self.glc.get_proc_address(addr).unwrap()
    }

    pub fn handle(&self) -> RawWindowHandle {
        let mut window_handle = XlibWindowHandle::empty();
        window_handle.window = self.window.as_raw();
        RawWindowHandle::Xlib(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(0)
    }

    pub fn swap_buffers(&self) {
        self.window.glx_swap_buffers();
    }

    pub fn run<F>(&self,callback: F)
        where
            F: Fn(crate::window::WindowEvent)
    {
        self.window.run(|event, control_flow| {
            match event {
                WindowEvent::Expose => {
                    callback(crate::window::WindowEvent::Update);
                }
            }
        })
    }
}
