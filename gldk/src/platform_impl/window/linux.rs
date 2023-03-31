use crate::window::WindowID;
use raw_window_handle::{RawWindowHandle, XlibWindowHandle};
use safex::glx::*;
use safex::xlib::*;

pub struct RWindow {
    display: Display,
    window: Window,
    cmap: ColorMap,
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
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
            cmap,
        }
    }

    pub fn handle(&self) -> RawWindowHandle {
        let mut window_handle = XlibWindowHandle::empty();
        window_handle.window = self.window.as_raw();
        RawWindowHandle::Xlib(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(0)
    }

    pub fn run(&self) {
        self.window.run(|event, control_flow| {})
    }
}
