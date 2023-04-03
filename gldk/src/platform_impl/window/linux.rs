use crate::window::WindowID;
use crate::{GLConfig, GLVersion};
use core::ffi::c_void;
use raw_window_handle::{RawWindowHandle, XlibWindowHandle};
use safex::glx::*;
use safex::xlib::*;
use gwl::window::*;

pub struct Props {
    glc: Option<GLXContext>,
}

#[derive(Clone,Copy)]
pub struct BuildAction {
    conf: GLConfig,
    props: *mut Props
}

impl WindowBuildAction for BuildAction {
    fn override_window_handle(&mut self) -> Option<WindowHandle> {
        let display = Display::open(None);
        let screen = Screen::default(&display);

        let (major, minor) = match self.conf.version {
            GLVersion::V3_0 => (3, 0),
            GLVersion::V3_1 => (3, 1),
            GLVersion::V3_2 => (3, 2),
            GLVersion::V3_3 => (3, 3),
            GLVersion::V4_0 => (4, 0),
            GLVersion::V4_1 => (4, 1),
            GLVersion::V4_2 => (4, 2),
            GLVersion::V4_3 => (4, 3),
            GLVersion::V4_4 => (4, 4),
            GLVersion::V4_5 => (4, 5),
            GLVersion::V4_6 => (4, 6),
        };

        let vi = glx_choose_visual(
            &display,
            &mut [
                GLX_CONTEXT_MAJOR_VERSION_ARB,
                major,
                GLX_CONTEXT_MINOR_VERSION_ARB,
                minor,
                GLX_RGBA,
                GLX_DEPTH_SIZE,
                24,
                GLX_DOUBLEBUFFER,
                GLX_NONE,
            ],
        )
            .unwrap();
        let window = Window::new_with_glx(
            &display, &screen, &vi, None, 0, 0, width, height, 1, vi.depth, 0, &vi,
        )
            .unwrap();

        let glc = GLXContext::create(&display, &vi, None, gl::TRUE as i32);
        glx_make_current(&display, &window, &glc);

        self.props.glc = Some(glc);

        window.map(&display);
        window.set_window_title(title);

        let handle = WindowHandle {
            window,
            display
        };

        Some(handle)
    }
}

pub struct RWindow {
    props: Props,
    inner: Window
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: GLConfig) -> Self {
        let props = Props {
            glc: None
        };
        
        let action = BuildAction {
            conf,
            props: addr_of_mut!(props)
        };

        let inner = WindowBuilder::new()
            .title(title)
            .width(width)
            .height(height)
            .build_action(Box::new(action))
            .build();




        Self {
            props,
            inner
        }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        self.props.glc.unwrap().get_proc_address(addr).unwrap() as *const c_void
    }

    pub fn handle(&self) -> RawWindowHandle {
        let mut window_handle = XlibWindowHandle::empty();
        //window_handle.window = self.window.as_raw();
        RawWindowHandle::Xlib(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(0)
    }

    pub fn swap_buffers(&self) {
        self.window.glx_swap_buffers();
    }

    pub fn make_current(&self) {
        //glx_make_current(&self.display, &self.window, &self.glc);
    }

    pub fn run<F>(&self, callback: F)
    where
        F: Fn(crate::window::WindowEvent),
    {
        self.inner.run(|event, control_flow| match event {
            WindowEvent::Expose => {
                callback(crate::window::WindowEvent::Update);
            }
        })
    }

    pub fn set_window_title(&self, title: &str) {
        //self.window.set_window_title(title);
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        //let geometry = self.window.get_geometry();

        (0,0)
    }

    pub fn get_window_pos(&self) -> (u32, u32) {
        //let geometry = self.window.get_geometry();

        (0,0)
    }
}
