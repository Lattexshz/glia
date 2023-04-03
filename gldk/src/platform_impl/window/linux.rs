use crate::window::WindowID;
use crate::{GLConfig, GLVersion};
use core::ffi::c_void;
use gwl::window::*;
use raw_window_handle::{RawWindowHandle, XlibWindowHandle};
use safex::glx::*;
use std::ptr::addr_of_mut;

pub struct Props {
    glc: Option<GLXContext>,
    width: u32,
    height: u32,
    title: String,
}

#[derive(Clone, Copy)]
pub struct BuildAction {
    conf: GLConfig,
    props: *mut Props,
}

impl WindowBuildAction for BuildAction {
    fn pre_init(&mut self) {}
    fn override_window_handle(&mut self) -> Option<WindowHandle> {
        let display = safex::xlib::Display::open(None);
        let screen = safex::xlib::Screen::default(&display);

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
        let window = unsafe {
            safex::xlib::Window::new_with_glx(
                &display,
                &screen,
                &vi,
                None,
                0,
                0,
                (*self.props).width,
                (*self.props).height,
                1,
                vi.depth,
                0,
                &vi,
            )
        }
        .unwrap();

        let glc = GLXContext::create(&display, &vi, None, gl::TRUE as i32);
        glx_make_current(&display, &window, &glc);

        unsafe {
            (*self.props).glc = Some(glc);
            window.map();
            window.set_window_title((*self.props).title.as_str());
        }

        let handle = WindowHandle { window, display };

        Some(handle)
    }

    fn window_created(&mut self, handle: &WindowInstance) {}
}

pub struct RWindow {
    props: Props,
    inner: Window,
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: GLConfig) -> Self {
        let mut props = Props {
            glc: None,
            width,
            height,
            title: title.to_owned(),
        };

        let action = BuildAction {
            conf,
            props: addr_of_mut!(props),
        };

        let inner = WindowBuilder::new()
            .title(title)
            .width(width)
            .height(height)
            .build_action(Box::new(action))
            .build();

        Self { props, inner }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        self.props
            .glc
            .as_ref()
            .unwrap()
            .get_proc_address(addr)
            .unwrap() as *const c_void
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
        let instance = self.inner.get_instance();
        instance.window.glx_swap_buffers();
    }

    pub fn make_current(&self) {
        let instance = self.inner.get_instance();
        glx_make_current(
            instance.display,
            instance.window,
            &self.props.glc.as_ref().unwrap(),
        );
    }

    pub fn run<F>(&self, callback: F)
    where
        F: Fn(crate::window::WindowEvent),
    {
        self.inner.run(|event, control_flow| match event {
            WindowEvent::Expose => {
                callback(crate::window::WindowEvent::Update);
            }

            _ => {}
        })
    }

    pub fn set_window_title(&self, title: &str) {
        //self.window.set_window_title(title);
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        //let geometry = self.window.get_geometry();

        (0, 0)
    }

    pub fn get_window_pos(&self) -> (u32, u32) {
        //let geometry = self.window.get_geometry();

        (0, 0)
    }
}
