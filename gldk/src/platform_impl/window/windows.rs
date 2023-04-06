use crate::sys::{glGetError, wgl, wgl_extra, WGLARBFunctions};
use crate::window::{KeyCode, WindowEvent, WindowID};
use core::ffi::c_void;
use std::ffi::CString;

use raw_window_handle::{RawWindowHandle, Win32WindowHandle};

use crate::{GLConfig, GLVersion};
use gwl::window::{Window, WindowBuildAction, WindowBuilder, WindowInstance};
use std::ptr::{addr_of, addr_of_mut, null_mut};

use winapi::um::winuser::*;

use crate::sys::wgl_extra::types::HGLRC;
use winapi::shared::minwindef::*;

use winapi::shared::windef::*;

use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::wingdi::{
    ChoosePixelFormat, SetPixelFormat, SwapBuffers, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW,
    PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
};
use winapi::um::winnt::PCSTR;
use crate::error::GLDKError;

pub struct Props {
    hwnd: Option<HWND>,
    hinstance: Option<HINSTANCE>,
    wgl: Option<WGLARBFunctions>,
    ctx: Option<HGLRC>,
}

pub struct BuildAction {
    conf: GLConfig,
    props: *mut Props,
    error: Option<GLDKError>
}

impl WindowBuildAction for BuildAction {
    fn pre_init(&mut self) {}

    fn window_created(&mut self, handle: &WindowInstance) {
        unsafe {
            (*self.props).hwnd = Some(handle.hwnd);
            (*self.props).hinstance = Some(handle.hinstance);
        }

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

        unsafe {
            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: PFD_MAIN_PLANE,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };

            let hdc = GetDC(handle.hwnd);
            let pixel_format = ChoosePixelFormat(hdc, addr_of!(pfd));

            if pixel_format == 0 {
                self.error = Some(GLDKError::new_unexpected("Pixel format is null.".to_owned()));
                return;
            }

            if SetPixelFormat(hdc, pixel_format, addr_of!(pfd)) == 0 {
                self.error = Some(GLDKError::new_unexpected("Can't set pixel format.".to_owned()));
                return;
            }

            let old_ctx = wgl::CreateContext(hdc as wgl::types::HDC);
            wgl::MakeCurrent(hdc as wgl::types::HDC, old_ctx);

            let att = [
                wgl_extra::CONTEXT_MAJOR_VERSION_ARB,
                major,
                wgl_extra::CONTEXT_MINOR_VERSION_ARB,
                minor,
                wgl_extra::CONTEXT_FLAGS_ARB,
                0,
                wgl_extra::CONTEXT_PROFILE_MASK_ARB,
                wgl_extra::CONTEXT_CORE_PROFILE_BIT_ARB,
                0,
            ];

            let func = crate::sys::WGLARBFunctions::load();
            let ctx =
                (func.wglCreateContextAttribsARB)(hdc as wgl_extra::types::HDC, null_mut(), &att);

            (*self.props).ctx = Some(ctx);
            (*self.props).wgl = Some(func);

            wgl::DeleteContext(old_ctx);
        }
    }
}

pub struct RWindow {
    props: Props,
    inner: Window,
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: GLConfig) -> Result<Self,GLDKError> {
        let mut props = Props {
            hwnd: None,
            hinstance: None,
            wgl: None,
            ctx: None,
        };

        let mut action = BuildAction {
            conf,
            props: addr_of_mut!(props),
            error: None,
        };

        let inner = WindowBuilder::new(Box::new(&mut action))
            .title(title)
            .width(width)
            .height(height)
            .build();

        match action.error.take() {
            None => {}
            Some(e) => {
                return Err(e);
            }
        }

        Ok(Self { props, inner })
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        let addr = CString::new(addr.as_bytes()).unwrap();
        let addr = addr.as_ptr();

        unsafe {
            let p = wgl::GetProcAddress(addr) as *const core::ffi::c_void;
            if !p.is_null() {
                return p;
            }
            let gl = GetModuleHandleA("Opengl32.dll".as_ptr() as *const i8);
            GetProcAddress(gl, addr as PCSTR) as *const _
        }
    }

    pub fn handle(&self) -> RawWindowHandle {
        let instance = self.inner.get_instance();
        let mut window_handle = Win32WindowHandle::empty();
        window_handle.hwnd = instance.hwnd as *mut c_void;
        window_handle.hinstance = instance.hinstance as *mut c_void;
        RawWindowHandle::Win32(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(self.inner.get_instance().hwnd as u64)
    }

    pub fn make_current(&self) {
        unsafe {
            let hwnd = self.props.hwnd.unwrap();
            let ctx = self.props.ctx.unwrap();
            wgl::MakeCurrent(
                GetDC(hwnd) as wgl::types::HDC,
                ctx as crate::sys::wgl::types::HGLRC,
            );
        }
    }

    pub fn swap_interval(&self,enable: bool) {
        let wgl = self.props.wgl.as_ref().unwrap();
        (wgl.wglSwapIntervalEXT)(enable as u32);
    }

    pub fn run<F>(&self, mut callback: F)
    where
        F: FnMut(WindowEvent),
    {
        self.inner.run(|event, _control_flow| match event {
            gwl::window::WindowEvent::Expose => {
                callback(WindowEvent::RedrawRequested);
            }
            gwl::window::WindowEvent::KeyDown(c) =>  {
                callback(WindowEvent::Keydown(KeyCode(c)))
            },
            gwl::window::WindowEvent::KeyUp(c) => callback(WindowEvent::Keyup(KeyCode(c))),
            gwl::window::WindowEvent::CloseRequested => {
                callback(WindowEvent::CloseRequested)
            }
        });
    }

    pub fn swap_buffers(&self) {
        unsafe {
            let hwnd = self.props.hwnd.unwrap();
            SwapBuffers(GetDC(hwnd));
        }
    }

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
