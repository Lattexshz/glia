use crate::sys::{wgl, wgl_extra};
use crate::window::{KeyCode, WindowEvent, WindowID};
use core::ffi::c_void;
use std::ffi::{CString, OsStr};

use std::os::windows::ffi::OsStrExt;

use raw_window_handle::{RawWindowHandle, Win32WindowHandle};

use crate::{GLConfig, GLVersion};
use std::ptr::{addr_of, addr_of_mut, null_mut};
use gwl::window::{Window, WindowBuildAction, WindowBuilder, WindowHandle, WindowInstance};

use winapi::um::winuser::*;

use crate::sys::wgl_extra::types::HGLRC;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::um::dwmapi::{DWM_BLURBEHIND, DwmEnableBlurBehindWindow};
use winapi::um::libloaderapi::{GetModuleHandleA, GetModuleHandleW, GetProcAddress};
use winapi::um::wingdi::{
    ChoosePixelFormat, SetPixelFormat, SwapBuffers, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW,
    PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
};

pub struct Props {
    hwnd: Option<HWND>,
    hinstance: Option<HINSTANCE>,
    ctx: Option<HGLRC>
}

#[derive(Clone,Copy)]
pub struct BuildAction {
    conf: GLConfig,
    props: *mut Props
}

impl WindowBuildAction for BuildAction {
    fn pre_init(&mut self) {

    }

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
            SetPixelFormat(hdc, pixel_format, addr_of!(pfd));

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


            unsafe {
                (*self.props).ctx = Some(ctx);
            }

            wgl::DeleteContext(old_ctx);
        }
    }
}

pub struct RWindow {
    props: Props,
    inner: Window
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: GLConfig) -> Self {
        let mut props = Props {
            hwnd: None,
            hinstance: None,
            ctx: None,
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
        let mut window_handle = Win32WindowHandle::empty();
        // window_handle.hwnd = self.hwnd as *mut c_void;
        // window_handle.hinstance = self.hinstance as *mut c_void;
        RawWindowHandle::Win32(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(0)
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

    pub fn run<F>(&self, callback: F)
    where
        F: Fn(WindowEvent),
    {
        unsafe {
            let mut message = core::mem::zeroed();

            while GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) != 0 {
                DispatchMessageW(&message);

                match message.message {
                    WM_PAINT => {
                        callback(WindowEvent::Update);
                    }

                    WM_CLOSE => {
                        callback(WindowEvent::CloseRequested);
                    }

                    WM_KEYDOWN => {
                        callback(WindowEvent::Keydown(KeyCode(message.wParam as u32)));
                    }

                    WM_KEYUP => {
                        callback(WindowEvent::Keyup(KeyCode(message.wParam as u32)));
                    }

                    _ => {}
                }
            }
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            let hwnd = self.props.hwnd.unwrap();
            SwapBuffers(GetDC(hwnd));
        }
    }

    pub fn set_window_title(&self, title: &str) {
        unsafe {
            let hwnd = self.props.hwnd.unwrap();
            SetWindowTextA(hwnd, title.as_ptr() as *const i8);
        }
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        unsafe {
            GetWindowRect(self.props.hwnd.unwrap(), &mut rect);
        }
        (rect.right.try_into().unwrap(),rect.bottom.try_into().unwrap())
    }

    pub fn get_window_pos(&self) -> (u32, u32) {
        // let mut rect = RECT {
        //     left: 0,
        //     top: 0,
        //     right: 0,
        //     bottom: 0,
        // };
        //
        // unsafe {
        //     GetWindowRect(self.hwnd, &mut rect);
        // }
        (0, 0)
    }
}
